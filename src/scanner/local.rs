use std::fs;
use std::path::{Path, PathBuf};
use std::sync::mpsc::Sender;
use anyhow::{Result, anyhow, Context};
use crate::scanner::{File, FileScanner};

pub struct LocalConfig {
    root: PathBuf,
    include: Vec<PathBuf>,
    exclude: Vec<PathBuf>,
    parallelism: u32,
}

pub struct LocalFile {
    path: PathBuf,
    absolute_path: PathBuf,
}

pub struct LocalFileScanner {
    config: LocalConfig,
}

impl LocalFileScanner {
    pub fn new(config: LocalConfig) -> LocalFileScanner {
        LocalFileScanner { config }
    }

    fn visit(&self, path: &Path, target: &Sender<LocalFile>) -> Result<()> {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            //todo: filter out the paths

            if path.is_dir() {
                self.visit(&path, target)?;
            } else {
                target.send(LocalFile::new(path.clone(), path.clone()))?;
            }
        }
        Ok(())
    }
}

impl FileScanner for LocalFileScanner {
    type F = LocalFile;

    fn scan(&self, target: Sender<Self::F>) -> Result<()> {
        self.visit(self.config.root.as_ref(), &target)
    }
}

impl LocalFile {
    pub fn new<P: AsRef<Path>, A: AsRef<Path>>(path: P, absolute_path: A) -> LocalFile {
        LocalFile {
            path: path.as_ref().to_path_buf(),
            absolute_path: absolute_path.as_ref().to_path_buf(),
        }
    }

}

impl File for LocalFile {
    fn path(&self) -> PathBuf {
        self.path.clone()
    }

    fn contents(&self) -> Result<String> {
        fs::read_to_string(&self.absolute_path)
            .context(format!("Unable to read from {}", self.absolute_path.display()))
    }
}

#[cfg(test)]
mod test {
    use std::env;
    use std::sync::mpsc;
    use std::sync::mpsc::Receiver;
    use super::*;

    #[test]
    fn scan_local_directory() {
        let config = LocalConfig {
            root: env::current_dir().unwrap(),
            include: vec!("tests/**/*.go".into()),
            exclude: vec!("**/*bad*".into()),
            parallelism: 1,
        };

        let scanner = LocalFileScanner::new(config);

        let (tx, rx): (Sender<LocalFile>, Receiver<LocalFile>) = mpsc::channel();

        scanner.scan(tx).unwrap();

        for x in rx {
            println!("p: {}, a: {}", x.path.display(), x.absolute_path.display());
        }
    }
}