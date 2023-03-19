use std::path::PathBuf;
use anyhow::Result;

#[derive(Debug, Eq, PartialEq)]
pub enum Event {
    ScanStarted,
    ParsingStarted(PathBuf),
    ParsingWarning(String),
    ParsingFinished(usize), // a number of found notes
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