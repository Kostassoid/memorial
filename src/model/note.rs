use std::path::PathBuf;
use super::handle::Handle;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct FileLocation {
    path: PathBuf,
    line: usize,
}

impl FileLocation {
    pub fn new<P: Into<PathBuf>>(path: P, line: usize) -> FileLocation {
        FileLocation {
            path: path.into(),
            line,
        }
    }
}

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
        Note {
            location,
            spans,
        }
    }

    pub fn spans(&self) -> &Vec<NoteSpan> {
        &self.spans
    }

    pub fn location(&self) -> &FileLocation {
        &self.location
    }
}