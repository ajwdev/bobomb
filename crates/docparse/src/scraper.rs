use anyhow::{anyhow, bail, Context, Result};
use ego_tree::{NodeId, Tree};
use itertools::Itertools;
use regex::Regex;
use scraper::{ElementRef, Html, Node, Selector};
use tracing::{debug, field, info, span, trace, Level};

use std::fmt::Write;
// rust seems to get confused about this import. We do end up using it but not directly
#[allow(unused_imports)]
use std::str::pattern::Pattern;

pub use crate::types::*;

#[derive(Debug, PartialEq, Eq)]
enum ParseState {
    Summary,
    StatusRegister,
    AddressModes,
    Footer,
}

impl Default for ParseState {
    fn default() -> Self {
        Self::Summary
    }
}

mod selector {
    use scraper::Selector;

    pub(super) fn h3() -> scraper::Selector {
        Selector::parse("h3").unwrap()
    }

    pub(super) fn a() -> scraper::Selector {
        Selector::parse("a").unwrap()
    }

    pub(super) fn td() -> scraper::Selector {
        Selector::parse("td").unwrap()
    }
}

fn trim_and_join_elem(elem: &ElementRef, op: &str) -> String {
    elem.text().join("").split("\n").map(|s| s.trim()).join(op)
}

fn trim_elem(elem: &ElementRef) -> String {
    trim_and_join_elem(elem, "")
}

fn u8_radix_elem(elem: &ElementRef, radix: u32) -> Result<u8> {
    let s = trim_and_join_elem(elem, "");
    u8::from_str_radix(s.as_str(), radix)
        .context(format!("unable to parse elem text as u8: |{}|", &s))
}

pub(crate) struct Scraper<'a> {
    buf: &'a str,
}

pub(crate) struct Iter {
    tree: Tree<Node>,
    refs: Vec<NodeId>,
    iter: usize,
}

impl Iter {
    pub fn new(scaper: &Scraper<'_>) -> Iter {
        let (tree, refs) = scaper.iter_graph();
        Self {
            tree,
            refs,
            iter: 0,
        }
    }
}

impl Iterator for Iter {
    type Item = Result<Instruction>;

    fn next(&mut self) -> Option<Self::Item> {
        let id = self.refs.get(self.iter);
        self.iter += 1;

        match id {
            Some(id) => self
                .tree
                .get(*id)
                .map(|n| ElementRef::wrap(n))
                .flatten()
                .map(|i| Scraper::extract_instruction(&i)),
            None => None,
        }
    }
}

impl<'a> Scraper<'a> {
    pub fn new(buf: &'a str) -> Self {
        Self { buf }
    }

    pub fn scrap(&self) -> Result<Docs> {
        let instructions = self.iter().collect::<Result<Vec<Instruction>>>()?;
        info!(count = instructions.len(), "Done scraping instructions");

        Ok(instructions.into())
    }

    pub fn iter(&self) -> Iter {
        Iter::new(self)
    }

    fn iter_graph(&self) -> (Tree<Node>, Vec<NodeId>) {
        let document = Html::parse_document(&self.buf);
        let refs = document
            .select(&selector::h3())
            .map(|e| e.id())
            .collect::<Vec<NodeId>>();

        (document.tree, refs)
    }

    fn extract_instruction(inst: &ElementRef) -> Result<Instruction> {
        let span = span!(
            Level::DEBUG,
            "extract_instruction",
            instruction = field::Empty
        );
        let _guard = span.enter();

        let mut instruction = Instruction {
            title: inst.text().collect(),
            ..Default::default()
        };

        // Grab the first "a" element. This will be our instruction name
        // Example
        // <h3>
        //     <a name="ADC"></a>
        //     ADC - Add with Carry
        // </h3>
        instruction.name = inst
            .select(&selector::a())
            .nth(0)
            .ok_or(anyhow!("unable to select nth(0) element"))?
            .attr("name")
            .ok_or(anyhow!("unable to select name from element"))?
            .to_owned();

        span.record("instruction", &instruction.name);
        debug!(%instruction.name, "Found instruction");

        // Set our scraper state to keep track of where we are
        let mut state = ParseState::default();
        macro_rules! transition_state {
            ($nxt:expr) => {
                debug!("Transitioning state {:?} --> {:?}", &state, $nxt);
                state = $nxt;
                trace!(?instruction, "Current State");
            };
        }

        let mut summary = String::new();
        let mut notes = String::new();
        let mut see_also: Vec<String> = Vec::new();

        for nref in inst.next_siblings() {
            let node_val = nref.value();

            // Early exit if we've encountered the next instruction
            if let Some(elem) = node_val.as_element() {
                if elem.name() == "h3" {
                    debug!("Encountered h3 element");
                    break;
                }
            }

            match node_val {
                Node::Element(elem) if state == ParseState::Summary => {
                    trace!(?state, ?elem.name, "Encountered Node::Element");

                    let s = nref
                        .children()
                        .inspect(|n| trace!(?state, node = ?n.value(), "Encountered child node"))
                        .flat_map(|c| c.value().as_text())
                        .map(|n| n.to_owned().replace("\n", " "))
                        .join("");

                    if s.contains("Processor Status after use") {
                        // Everything past this point will be register stuff
                        trace!("Encountered 'Processor Status after use' text element");
                        transition_state!(ParseState::StatusRegister);
                        continue;
                    }

                    write!(summary, "{}", &s)?;
                    trace!(text = %s, "Appended to summary buffer from element node")
                }
                Node::Text(txt) if state == ParseState::Summary => {
                    trace!(?state, ?txt.text, "Encountered Node::Text");

                    write!(summary, "{}", &txt.text)?;
                    trace!(text = %txt.text, ?state, "Appended to summary buffer from text node");

                    // Lets peek ahead and see if the next sibling contains a table. If so we
                    // can assume its for status register flags. Some instructions such as CLC
                    // lack the "Processor Status after use" text.
                    if let Some(nxt) = nref.next_sibling() {
                        let chld = nxt
                            .first_child()
                            .map(|e| e.value().as_element().map(|n| n.name()))
                            .flatten()
                            .unwrap_or("");

                        if chld == "table" {
                            trace!("Encountered sibling with child table");
                            transition_state!(ParseState::StatusRegister);
                            continue;
                        }
                    }
                }
                Node::Element(elem) if state == ParseState::StatusRegister => {
                    trace!(?state, ?elem.name, "Encountered Node::Element");

                    // TODO Macro this. Tried making a helper function but nref is a
                    // ego_tree::NodeRef. I dont feel like pulling into dependency at the toplevel
                    // just for this.
                    if let Some(table_ref) = nref.first_child() {
                        if table_ref
                            .value()
                            .as_element()
                            .inspect(|elem| trace!(?state, ?elem.name, "Encountered child element"))
                            .map(|e| e.name())
                            .unwrap_or("")
                            != "table"
                        {
                            // Not a table, move on
                            continue;
                        }
                    };

                    let table_ref = ElementRef::wrap(nref.first_child().unwrap()).unwrap();
                    for row in table_ref.select(&Selector::parse("tr").unwrap()) {
                        trace!(?state, node = ?row.value().name, "Encountered child element ref");
                        Self::extract_status_row(&row, &mut instruction.status_register)?;
                    }

                    transition_state!(ParseState::AddressModes);
                }
                Node::Element(elem) if state == ParseState::AddressModes => {
                    trace!(?state, ?elem.name, "Encountered Node::Element");

                    // TODO Macro this. See above
                    if let Some(table_ref) = nref.first_child() {
                        if table_ref
                            .value()
                            .as_element()
                            .inspect(|elem| trace!(?state, ?elem.name, "Encountered child element"))
                            .map(|e| e.name())
                            .unwrap_or("")
                            != "table"
                        {
                            // Not a table, move on
                            continue;
                        }
                    };

                    let table_ref = ElementRef::wrap(nref.first_child().unwrap()).unwrap();
                    // NOTE Skip 1 to skip the table header
                    for row in table_ref.select(&Selector::parse("tr").unwrap()).skip(1) {
                        trace!(?state, node = ?row.value().name, "Encountered child element ref");
                        Self::extract_address_row(&row, &mut instruction.address_modes)?;
                    }

                    transition_state!(ParseState::Footer);
                }
                Node::Element(elem) if state == ParseState::Footer => {
                    trace!(?state, ?elem.name, "Encountered Node::Element");

                    let s = nref
                        .children()
                        .inspect(|n| trace!(?state, node = ?n.value(), "Encountered child node"))
                        .flat_map(|c| c.value().as_text())
                        .map(|n| {
                            n.trim_start_matches(&[' ', ':', '\t'])
                                .trim_start_matches("NB:")
                                .trim_end()
                                .replace("\n", " ")
                        })
                        .join("");

                    if s != "See also:" {
                        write!(notes, "{}", &s)?;
                        trace!(text = %s, "Appended to notes buffer from element node");
                    }

                    ElementRef::wrap(nref)
                        .unwrap()
                        .select(&selector::a())
                        .inspect(|e| trace!(?state, node = ?e.value().name, "Encountered child element ref"))
                        .map(|e| trim_and_join_elem(&e, ""))
                        .for_each(|link| see_also.push(link));
                }
                Node::Text(txt) => {
                    trace!(?state, ?txt.text, "Encountered *unmatched* Node::Text");
                }
                Node::Element(elem) => {
                    trace!(?state, ?elem.name, "Encountered *unmatched* Node::Element");
                }
                node @ _ => {
                    trace!(?state, ?node, "Encountered *unmatched* Node");
                }
            };
        }

        instruction.summary = summary.trim().to_string();
        instruction.notes = notes.trim().to_string();
        instruction.see_also = see_also;

        Ok(instruction)
    }

    fn extract_status_row(row_ref: &ElementRef, status: &mut StatusRegisters) -> Result<()> {
        let span = span!(Level::DEBUG, "extract_status_row");
        let _guard = span.enter();

        let cols = row_ref
            .select(&selector::td())
            .take(3)
            .collect::<Vec<ElementRef>>();
        let (register_elem, txt_elem) = (cols[0], cols[2]);

        let register = trim_elem(&register_elem);
        let txt = txt_elem
            .text()
            .flat_map(|txt| {
                if txt == "Not affected" {
                    None
                } else {
                    Some(txt.trim().to_string())
                }
            })
            .nth(0);

        trace!(?register, ?txt, "Extracted status row");

        match register.as_str() {
            "C" => status.carry = txt,
            "Z" => status.zero = txt,
            "I" => status.interrupt = txt,
            "D" => status.decimal_mode = txt,
            "B" => status.break_command = txt,
            "V" => status.overflow = txt,
            "N" => status.negative = txt,
            _ => bail!("unknown status type column {}", &register),
        };

        Ok(())
    }

    fn extract_address_row(row_ref: &ElementRef, address_modes: &mut AddressModes) -> Result<()> {
        use std::u8;

        let cols = row_ref.select(&selector::td()).take(4).collect_tuple();

        let (mode_elem, opcode_elem, length_elem, cycles_elem) =
            cols.ok_or(anyhow!("expected some, got none"))?;

        // Mode is the name of address mode
        let mode = trim_and_join_elem(&mode_elem, " ");
        // Opcode is a hex string in 6502 format (i.e $BE vs 0xBE)
        let opcode = trim_elem(&opcode_elem);
        // Length is number of bytes used for the operation
        let length = u8_radix_elem(&length_elem, 16)?;
        // Finally cycles which is either a single number or a number and a description of when
        // extra cycles are added.
        let cycles_str = trim_and_join_elem(&cycles_elem, ", ");
        let (cycles, extra_cycles) = if cycles_str.chars().count() > 1 {
            // I think we have extra cycles
            let re = Regex::new(r"([0-9]) ?\((.+)\)$").unwrap();

            let Some((_, [ctmp, extra_cycles])) =
                re.captures(cycles_str.trim()).map(|caps| caps.extract())
            else {
                bail!("regex did not match")
            };

            (
                u8::from_str_radix(ctmp, 10)?,
                Some(extra_cycles.to_string()),
            )
        } else {
            (u8::from_str_radix(&cycles_str, 10)?, None)
        };

        let details = AddressModeDetail {
            opcode,
            byte_len: length,
            cycles,
            extra_cycles,
        };

        match mode.as_str() {
            "Accumulator" => address_modes.accumulator = Some(details),
            "Immediate" => address_modes.immediate = Some(details),
            "Zero Page" => address_modes.zero_page = Some(details),
            "Zero Page,X" => address_modes.zero_page_x = Some(details),
            "Zero Page,Y" => address_modes.zero_page_y = Some(details),
            "Absolute" => address_modes.absolute = Some(details),
            "Absolute,X" => address_modes.absolute_x = Some(details),
            "Absolute,Y" => address_modes.absolute_y = Some(details),
            "(Indirect,X)" => address_modes.indirect_x = Some(details),
            "(Indirect),Y" => address_modes.indirect_y = Some(details),
            "Relative" => address_modes.relative = Some(details),
            "Implied" => address_modes.implied = Some(details),
            "Indirect" => address_modes.indirect = Some(details),
            _ => bail!("unknown status type column |{}|", &mode),
        };

        Ok(())
    }
}
