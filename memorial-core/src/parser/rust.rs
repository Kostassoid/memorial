use pest::Parser as P;
use pest_derive::Parser;
use anyhow::Result;
use pest::iterators::Pair;
use memorial_macros::FileParser;
use crate::parser::{Quote, FileParser};

#[derive(Parser, FileParser)]
#[grammar = "src/parser/rust.pest"]
pub struct RustParser;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_rust() {
        let parsed = RustParser{}.parse_from_str(r#"
        /*
        Block comment
        is long
        */
        fn test_fun() {
          let simple_string = "This is /* not a comment */";
          let fancy_string = r####"
          This also is /* not a comment */
          "\\####;
          // Inline comment
          // Another inline comment
        }
       "#).unwrap();

        let expected = vec!(
            Quote { body: "Block comment\nis long".to_string(), line: 2 },
            Quote { body: "Inline comment".to_string(), line: 11 },
            Quote { body: "Another inline comment".to_string(), line: 12 }
        );

        assert_eq!(expected, parsed)
    }
}