use super::grammar::CommandParser;

use lazy_static::lazy_static;
lazy_static! {
    pub static ref PARSER: CommandParser = CommandParser::new();
}

#[cfg(test)]
mod test {
    use super::PARSER;

    #[test]
    fn parse_examine_ok() {
        [
            "x 10",
            "x 0xBEEF",
            "x 0xbeef",
            "x $PC",
            "x/10i $PC",
            "x/10i $PC+10-1",
        ]
        .iter()
        .for_each(|s| {
            assert!(
                PARSER.parse(s).is_ok(),
                "could not parse command: \"{}\"",
                s
            );
        })
    }
}
