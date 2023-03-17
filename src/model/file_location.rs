use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use anyhow::{anyhow, Result};
use url::Url;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum FilePath {
    Relative(PathBuf),
    AbsoluteUrl(Url),
}

impl Display for FilePath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FilePath::Relative(pb) => f.write_str(pb.to_str().unwrap()),
            FilePath::AbsoluteUrl(url) => f.write_str(url.as_str()),
        }?;
        Ok(())
    }
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

    pub fn path(&self) -> &FilePath {
        &self.path
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn is_relative(&self) -> bool {
        matches!(&self.path, FilePath::Relative(_))
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

    pub fn replace_path(&mut self, path: FilePath) {
        self.path = path;
    }
}
