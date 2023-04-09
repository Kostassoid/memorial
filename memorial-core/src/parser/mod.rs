use anyhow::Result;

pub mod csharp;
pub mod go;
pub mod java;
pub mod javascript;
pub mod kotlin;
pub mod protobuf;
pub mod rust;

#[derive(Debug, Eq, PartialEq)]
pub struct Quote {
    pub body: String,
    pub line: usize,
}

pub trait FileParser {
    fn parse_from_str(&self, source: &str) -> Result<Vec<Quote>>;
}
