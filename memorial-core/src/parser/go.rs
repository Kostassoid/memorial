use crate::parser::{FileParser, Quote};
use anyhow::Result;
use memorial_macros::FileParser;
use pest::iterators::Pair;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(Parser, FileParser)]
#[grammar = "src/parser/go.pest"]
pub struct GoParser;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_go() {
        let parsed = GoParser {}
            .parse_from_str(
                r#"
        package test

        /*
        Block comment
        is long
        */
        func TestFun() {
          _ = "This is /* not a comment */"
          // Inline comment
          // Another inline comment
        }
       "#,
            )
            .unwrap();

        let expected = vec![
            Quote {
                body: "Block comment\nis long".to_string(),
                line: 4,
            },
            Quote {
                body: "Inline comment".to_string(),
                line: 10,
            },
            Quote {
                body: "Another inline comment".to_string(),
                line: 11,
            },
        ];

        assert_eq!(expected, parsed)
    }
}
