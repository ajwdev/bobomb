use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Docs {
    pub instructions: Vec<Instruction>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_at: Option<String>,
}

impl From<Vec<Instruction>> for Docs {
    fn from(v: Vec<Instruction>) -> Self {
        Self {
            instructions: v,
            ..Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Instruction {
    pub name: String,
    pub title: String,
    pub summary: String,
    pub status_register: StatusRegisters,
    pub address_modes: AddressModes,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub notes: String,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub see_also: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AddressModeDetail {
    pub opcode: String,
    pub byte_len: u8,
    pub cycles: u8,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_cycles: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AddressModes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implied: Option<AddressModeDetail>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub immediate: Option<AddressModeDetail>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative: Option<AddressModeDetail>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub accumulator: Option<AddressModeDetail>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub absolute: Option<AddressModeDetail>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub absolute_x: Option<AddressModeDetail>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub absolute_y: Option<AddressModeDetail>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub zero_page: Option<AddressModeDetail>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub zero_page_x: Option<AddressModeDetail>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub zero_page_y: Option<AddressModeDetail>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub indirect: Option<AddressModeDetail>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub indirect_x: Option<AddressModeDetail>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub indirect_y: Option<AddressModeDetail>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct StatusRegisters {
    #[serde(rename = "C")]
    pub carry: Option<String>,

    #[serde(rename = "Z")]
    pub zero: Option<String>,

    #[serde(rename = "I")]
    pub interrupt: Option<String>,

    #[serde(rename = "D")]
    pub decimal_mode: Option<String>,

    #[serde(rename = "B")]
    pub break_command: Option<String>,

    #[serde(rename = "V")]
    pub overflow: Option<String>,

    #[serde(rename = "N")]
    pub negative: Option<String>,
}
