use pest::Parser as P;
use pest_derive::Parser;
use anyhow::{anyhow, Result};
use crate::collector::QuoteSpan;
use crate::model::handle::Handle;

#[derive(Parser)]
#[grammar = "src/collector/quote.pest"]
pub struct QuoteParser;

impl QuoteParser {
    pub fn parse_from_str(source: &str) -> Result<Vec<QuoteSpan>> {
        let parsed = QuoteParser::parse(Rule::root, source)?;
        let spans: Result<Vec<QuoteSpan>> = parsed.into_iter()
            .map(|p| {
                match p.as_rule() {
                    Rule::handle =>
                        Ok(QuoteSpan::Link(Handle::build_from_str(p.into_inner().as_str())?)),
                    Rule::attr => {
                        let mut inner = p.into_inner();
                        Ok(QuoteSpan::Attribute(
                            inner.next().ok_or(anyhow!("Missing attribute key"))?.as_str().to_string(),
                            inner.next().ok_or(anyhow!("Missing attribute value"))?.as_str().to_string()
                        ))
                    },
                    Rule::text => Ok(QuoteSpan::Text(p.as_str().to_string())),
                    _ => { unreachable!() }
                }
            })
            .collect();

        Ok(spans?)
    }
}

#[cfg(test)]
mod tests {
    use crate::model::note::Note;
    use super::*;

    #[test]
    fn parse_quote() {
        let parsed = QuoteParser::parse_from_str(r#"
            @[Domain/Accumulator/Invariants]{alias:Domain rules}
            The accumulated value is always increasing when collecting new values.
            See @[Domain/Other/Rule] for more details.
       "#).unwrap();

        let expected = vec!(
            QuoteSpan::Link(Handle::build_from_parts(vec!("Domain", "Accumulator", "Invariants")).unwrap()),
            QuoteSpan::Attribute("alias".to_string(), "Domain rules".to_string()),
            QuoteSpan::Text("The accumulated value is always increasing when collecting new values.\n            See".to_string()),
            QuoteSpan::Link(Handle::build_from_parts(vec!("Domain", "Other", "Rule")).unwrap()),
            QuoteSpan::Text("for more details.".to_string()),
        );

        assert_eq!(expected, parsed)
    }
}