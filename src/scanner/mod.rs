use std::path::{Path, PathBuf};
use std::sync::mpsc::Sender;
use anyhow::{Result, anyhow};

pub(crate) mod config;
mod local;
mod path_filter;

trait File {
    fn path(&self) -> PathBuf;
    fn contents(&self) -> Result<String>;
}

trait FileScanner {
    type F: File;

    fn scan(&self, target: Sender<Self::F>) -> Result<()>;
}

