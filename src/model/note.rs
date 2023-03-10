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
    handle: Handle,
    location: FileLocation,
    body: Option<String>,
    mentions: Vec<Handle>
}

impl Note {
    pub fn new(handle: Handle, location: FileLocation, body: Option<String>, mentions: Vec<Handle>) -> Note {
        Note {
            handle,
            location,
            body,
            mentions,
        }
    }

    pub fn handle(&self) -> &Handle {
        &self.handle
    }
}