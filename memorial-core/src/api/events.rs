use std::path::PathBuf;
use anyhow::Result;

#[derive(Debug, Eq, PartialEq)]
pub enum Event {
    ScanStarted,
    UnknownFileTypeEncountered(PathBuf),
    ParsingStarted(PathBuf),
    ParsingFailed(PathBuf, String),
    ParsingFinished(PathBuf, usize), // a number of found notes
    ScanFinished,
}

pub trait EventHandler {
    fn send(&mut self, event: Event) -> Result<()>;
}

pub struct StubEventHandler {
    pub events: Vec<Event>,
}

impl StubEventHandler {
    pub fn new() -> StubEventHandler {
        StubEventHandler { events: vec![] }
    }
}

impl EventHandler for StubEventHandler {
    fn send(&mut self, event: Event) -> Result<()> {
        self.events.push(event);
        Ok(())
    }
}