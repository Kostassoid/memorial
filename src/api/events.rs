use std::path::PathBuf;
use anyhow::Result;

#[derive(Debug)]
pub enum Event {
    ScanStarted,
    ParsingStarted(PathBuf),
    ParsingWarning(String),
}

pub trait EventHandler {
    fn send(&mut self, event: Event) -> Result<()>;
}

pub struct StubEventHandler {
    pub events: Vec<Event>
}

impl EventHandler for StubEventHandler {
    fn send(&mut self, event: Event) -> Result<()> {
        self.events.push(event);
        Ok(())
    }
}