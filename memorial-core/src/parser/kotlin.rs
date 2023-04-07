use anyhow::Result;
use pest::iterators::Pair;
use pest::Parser as P;
use pest_derive::Parser;

use memorial_macros::FileParser;

use crate::parser::{FileParser, Quote};

#[derive(Parser, FileParser)]
#[grammar = "src/parser/kotlin.pest"]
pub struct KotlinParser;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_kotlin() {
        let parsed = KotlinParser {}
            .parse_from_str(
                r#"
        import java.time.LocalDateTime

        /*
        Block comment
        is long
        */
        fun testFun() {
          val x = "This is /* not a comment */"
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
