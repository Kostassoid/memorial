use std::fs;
use std::path::{Path, PathBuf};
use std::sync::mpsc::Sender;
use anyhow::{Result, Context};
use crate::scanner::{File, FileScanner};
use crate::scanner::path_filter::PathFilter;

pub struct LocalConfig {
    root: PathBuf,
    include: Vec<String>,
    exclude: Vec<String>,
}

impl LocalConfig {
    pub fn new<P: AsRef<Path>>(root: P, include: Vec<String>, exclude: Vec<String>) -> LocalConfig {
        LocalConfig {
            root: root.as_ref().to_path_buf(),
            include,
            exclude
        }
    }
}

pub struct LocalFile {
    local_path: PathBuf,
    absolute_path: PathBuf,
}

pub struct LocalFileScanner {
    config: LocalConfig,
    filter: PathFilter,
}

impl LocalFileScanner {
    pub fn new(config: LocalConfig) -> Result<LocalFileScanner> {
        let filter = PathFilter::from_glob(
            &config.include,
            &config.exclude,
        )?;

        Ok(LocalFileScanner {
            config,
            filter,
        })
    }

    fn visit(&self, path: &Path, target: &Sender<LocalFile>) -> Result<()> {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            let local_path = path.strip_prefix(&self.config.root)?;

            if path.is_dir() {
                self.visit(&path, target)?;
            } else {
                if !self.filter.is_allowed(local_path.as_os_str()) {
                    continue;
                }

                target.send(LocalFile::new(
                    local_path,
                    path.clone(),
                ))?;
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
    pub fn new<P: AsRef<Path>, A: AsRef<Path>>(local_path: P, absolute_path: A) -> LocalFile {
        LocalFile {
            local_path: local_path.as_ref().to_path_buf(),
            absolute_path: absolute_path.as_ref().to_path_buf(),
        }
    }

}

impl File for LocalFile {
    fn path(&self) -> PathBuf {
        self.local_path.clone()
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
            include: vec!("src/tests/**/*.go".into()),
            exclude: vec!("**/*bad*".into()),
        };

        let scanner = LocalFileScanner::new(config).unwrap();

        let (tx, rx): (Sender<LocalFile>, Receiver<LocalFile>) = mpsc::channel();

        scanner.scan(tx).unwrap();

        let valid_files: Vec<_> = rx.into_iter().collect();

        assert_eq!(2, valid_files.len());
        assert_eq!(
            vec!(
                r"src\tests\cases\go\app.go",
                r"src\tests\cases\go\domain.go",
            ),
            valid_files.iter().map(|f| f.local_path.to_str().unwrap()).collect::<Vec<_>>(),
        );
    }
}