use crate::model::handle::Handle;

pub mod collector;
pub mod file_matcher;
mod quote_parser;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum QuoteSpan {
    Link(Handle),
    Attribute(String, String),
    Text(String),
}