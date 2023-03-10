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
pub struct Note {
    location: FileLocation,
    body: Option<String>,
    mentions: Vec<Handle>
}

impl Note {
    pub fn new(location: FileLocation, body: Option<String>, mentions: Vec<Handle>) -> Note {
        Note {
            location,
            body,
            mentions,
        }
    }
}