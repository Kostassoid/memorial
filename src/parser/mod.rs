pub mod go;

use anyhow::Result;

// Naming it "comment" seems too limiting
#[derive(Debug, Eq, PartialEq)]
pub struct Quote {
    pub body: String,
    pub line: usize,
}

pub trait QuoteParser {
    fn extract_from_str(&self, source: &str) -> Result<Vec<Quote>>;
}