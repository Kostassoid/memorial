use anyhow::Result;
use pest::iterators::Pair;
use pest::Parser as P;
use pest_derive::Parser;

use memorial_macros::FileParser;

use crate::parser::{FileParser, Quote};

#[derive(Parser, FileParser)]
#[grammar = "src/parser/javascript.pest"]
pub struct JavaScriptParser;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_javascript() {
        let parsed = JavaScriptParser {}
            .parse_from_str(
                r#"
        import "module-name";
        
        /*
        Block comment
        is long
        */
        function test(){  
          var x = "This is /* not a comment */";
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
