use pest::Parser as P;
use pest_derive::Parser;
use anyhow::Result;
use pest::iterators::Pair;
use crate::parser::{Quote, FileParser};

#[derive(Parser)]
#[grammar = "src/parser/rust.pest"]
pub struct RustParser;

impl RustParser {
    //todo: extract or make part of the pest rules
    fn rule_to_quote(pair: Pair<Rule>) -> Quote {
        let line = pair.line_col().0;
        let indent = pair.line_col().1 - 1;

        let body = pair.into_inner().as_str()
            .replace(&format!("\n{}", " ".repeat(indent)), "\n")
            .replace(&format!("\n{}", "\t".repeat(indent)), "\n")
            .trim()
            .to_string();

        Quote {
            line,
            body,
        }
    }
}

impl FileParser for RustParser {
    fn parse_from_str(&self, source: &str) -> Result<Vec<Quote>> {
        let parsed = RustParser::parse(Rule::root, source)?;
        Ok(parsed
            .filter_map(|p| {
                match p.as_rule() {
                    Rule::COMMENT => Some(RustParser::rule_to_quote(p)),
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