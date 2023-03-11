use pest::Parser as P;
use pest_derive::Parser;
use anyhow::{anyhow, Result};
use crate::model::handle::Handle;
use crate::model::note::NoteSpan;

#[derive(Parser)]
#[grammar = "src/collector/note.pest"]
pub struct NoteParser;

impl NoteParser {
    pub fn parse_from_str(source: &str) -> Result<Vec<NoteSpan>> {
        let parsed = NoteParser::parse(Rule::root, source)?;
        let spans: Result<Vec<NoteSpan>> = parsed.into_iter()
            .map(|p| {
                match p.as_rule() {
                    Rule::handle =>
                        Ok(NoteSpan::Link(Handle::build_from_str(p.into_inner().as_str())?)),
                    Rule::attr => {
                        let mut inner = p.into_inner();
                        Ok(NoteSpan::Attribute(
                            inner.next().ok_or(anyhow!("Missing attribute key"))?.as_str().to_string(),
                            inner.next().ok_or(anyhow!("Missing attribute value"))?.as_str().to_string()
                        ))
                    },
                    Rule::text => Ok(NoteSpan::Text(p.as_str().to_string())),
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
    fn parse_note() {
        let parsed = NoteParser::parse_from_str(r#"
            @[Domain/Accumulator/Invariants]{alias:Domain rules}
            The accumulated value is always increasing when collecting new values.
            See @[Domain/Other/Rule] for more details.
       "#).unwrap();

        let expected = vec!(
            NoteSpan::Link(Handle::build_from_parts(vec!("Domain", "Accumulator", "Invariants")).unwrap()),
            NoteSpan::Attribute("alias".to_string(), "Domain rules".to_string()),
            NoteSpan::Text("The accumulated value is always increasing when collecting new values.\n            See".to_string()),
            NoteSpan::Link(Handle::build_from_parts(vec!("Domain", "Other", "Rule")).unwrap()),
            NoteSpan::Text("for more details.".to_string()),
        );

        assert_eq!(expected, parsed)
    }
}