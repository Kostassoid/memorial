use pest::Parser as P;
use pest_derive::Parser;
use anyhow::Result;
use crate::parser::{Quote, QuoteParser};

#[derive(Parser)]
#[grammar = "src/parser/go.pest"]
pub struct GoParser;

impl QuoteParser for GoParser {
    fn extract_from_str(&self, source: &str) -> Result<Vec<Quote>> {
        let parsed = GoParser::parse(Rule::root, source)?;
        Ok(parsed
            .filter_map(|p| {
                match p.as_rule() {
                    Rule::COMMENT => Some(
                        Quote {
                            line: p.line_col().0,
                            body: p.into_inner().as_str().trim().to_string(),
                        }
                    ),
                    _ => None
                }
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_go() {
        let parsed = GoParser{}.extract_from_str(r#"
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
       "#).unwrap();

        let expected = vec!(
            Quote { body: "Block comment\n        is long".to_string(), line: 4 },
            Quote { body: "Inline comment".to_string(), line: 10 },
            Quote { body: "Another inline comment".to_string(), line: 11 }
        );

        assert_eq!(expected, parsed)
    }
}