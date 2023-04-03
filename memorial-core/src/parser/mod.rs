pub mod go;
pub mod rust;

use anyhow::Result;

// Naming it "comment" seems too limiting
#[derive(Debug, Eq, PartialEq)]
pub struct Quote {
    pub body: String,
    pub line: usize,
}

pub trait FileParser {
    fn parse_from_str(&self, source: &str) -> Result<Vec<Quote>>;
}
