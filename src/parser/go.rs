use pest::Parser as P;
use pest_derive::Parser;
use anyhow::Result;
use pest::iterators::Pair;
use crate::parser::{Quote, FileParser};

#[derive(Parser)]
#[grammar = "src/parser/go.pest"]
pub struct GoParser;

impl GoParser {
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

impl FileParser for GoParser {
    fn parse_from_str(&self, source: &str) -> Result<Vec<Quote>> {
        let parsed = GoParser::parse(Rule::root, source)?;
        Ok(parsed
            .filter_map(|p| {
                match p.as_rule() {
                    Rule::COMMENT => Some(GoParser::rule_to_quote(p)),
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
        let parsed = GoParser{}.parse_from_str(r#"
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
            Quote { body: "Block comment\nis long".to_string(), line: 4 },
            Quote { body: "Inline comment".to_string(), line: 10 },
            Quote { body: "Another inline comment".to_string(), line: 11 }
        );

        assert_eq!(expected, parsed)
    }
}