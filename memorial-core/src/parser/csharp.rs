use anyhow::Result;
use pest::iterators::Pair;
use pest::Parser as P;
use pest_derive::Parser;

use memorial_macros::FileParser;

use crate::parser::{FileParser, Quote};

#[derive(Parser, FileParser)]
#[grammar = "src/parser/csharp.pest"]
pub struct CSharpParser;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_csharp() {
        let parsed = CSharpParser {}
            .parse_from_str(
                r#"
        using System;

        /*
        Block comment
        is long
        */
        class Test {
          void TestFun() {
            string x = "This is /* not a comment */";
            // Inline comment
            // Another inline comment
          }
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
                line: 11,
            },
            Quote {
                body: "Another inline comment".to_string(),
                line: 12,
            },
        ];

        assert_eq!(expected, parsed)
    }
}
