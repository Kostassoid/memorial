use anyhow::Result;
use pest::iterators::Pair;
use pest::Parser as P;
use pest_derive::Parser;

use memorial_macros::FileParser;

use crate::parser::{FileParser, Quote};

#[derive(Parser, FileParser)]
#[grammar = "src/parser/protobuf.pest"]
pub struct ProtobufParser;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_protobuf() {
        let parsed = ProtobufParser {}
            .parse_from_str(
                r#"
        syntax = "proto3";
        
        /*
        Block comment
        is long
        */
        message Test { // Inline comment
            reserved "//nope";
            string foo = 1;
        }
        // Another inline comment
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
                line: 8,
            },
            Quote {
                body: "Another inline comment".to_string(),
                line: 12,
            },
        ];

        assert_eq!(expected, parsed)
    }
}
