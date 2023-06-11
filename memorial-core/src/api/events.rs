use std::path::PathBuf;

use anyhow::Result;

/*@[Core/API]: Event-based callback system allows to decouple core logic from UI without complicating
abstractions. Also works really well in unit tests.

Primarily used by @[Core/Collector] so far but can be extended easily if needed.
 */
#[derive(Debug, Eq, PartialEq)]
pub enum Event {
    ScanStarted,
    UnknownFileTypeEncountered(PathBuf),
    ParsingStarted(PathBuf),
    ParsingFailed(PathBuf, String),
    ParsingFinished(PathBuf, usize),
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
