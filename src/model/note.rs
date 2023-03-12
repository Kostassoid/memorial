use std::path::PathBuf;
use url::Url;
use anyhow::{anyhow, Result};
use super::handle::Handle;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum FilePath {
    Relative(PathBuf),
    AbsoluteUrl(Url),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct FileLocation {
    path: FilePath,
    line: usize,
}

impl FileLocation {
    pub fn new_relative<P: Into<PathBuf>>(path: P, line: usize) -> FileLocation {
        FileLocation {
            path: FilePath::Relative(path.into()),
            line,
        }
    }

    pub fn is_relative(&self) -> bool {
        if let FilePath::Relative(_) = &self.path {
            true
        } else {
            false
        }
    }

    pub fn to_absolute_url(&self, prefix: &Url) -> Result<FileLocation> {
        match &self.path {
            FilePath::Relative(pb) => Ok(FileLocation {
                path: FilePath::AbsoluteUrl(
                    prefix.join(pb.to_str().ok_or(anyhow!("Can't convert path to string"))?)?
                ),
                line: self.line,
            }),
            _ => Err(anyhow!("Can't convert non-relative path to absolute Url"))
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