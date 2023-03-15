use std::path::PathBuf;

#[derive(Debug)]
pub enum Event {
    ScanStarted(PathBuf),
    ParsingStarted(PathBuf),
    ParsingWarning(String),
}