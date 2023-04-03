use super::handle::Handle;
use crate::model::file_location::FileLocation;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum NoteSpan {
    Link(Handle),
    Text(String),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Note {
    location: FileLocation,
    spans: Vec<NoteSpan>,
}

impl Note {
    pub fn new(location: FileLocation, spans: Vec<NoteSpan>) -> Note {
        Note { location, spans }
    }

    pub fn spans(&self) -> &Vec<NoteSpan> {
        &self.spans
    }

    pub fn location(&self) -> &FileLocation {
        &self.location
    }

    pub fn location_mut(&mut self) -> &mut FileLocation {
        &mut self.location
    }
}
