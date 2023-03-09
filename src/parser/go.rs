use pest::Parser as P;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "src/parser/go.pest"]
struct GoParser;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_go() {

        let parsed = GoParser::parse(Rule::item, r"
        package test

        /*
        Block comment
        */
        func TestFun() {
          // Inline comment
          // Another inline comment
        }
       ").unwrap();

        // let mut block = vec![];
        // let mut inline = vec![];

        parsed.for_each(|p| {
            match p.as_rule() {
                Rule::block_comment => println!("Block: {}", p.as_str()),
                Rule::line_comment => println!("Inline: {}", p.as_str()),
                Rule::COMMENT => println!("Comment: {}", p.as_str()),
                _ => {}
            }
        });

        // assert_eq!("???", parsed.to_string())
    }
}