use std::fmt::{Display, Formatter};
use std::path::PathBuf;
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
    title: String,
    path: FilePath,
    line: usize,
}

impl FileLocation {
    pub fn new_relative<P: Into<PathBuf>>(path: P, line: usize) -> FileLocation {
        let pb = path.into();
        FileLocation {
            title: pb.to_str().unwrap().to_string(),
            path: FilePath::Relative(pb),
            line,
        }
    }

    pub fn title(&self) -> &str {
        &self.title
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

    pub fn replace_path(&mut self, path: FilePath) {
        self.path = path;
    }
}
