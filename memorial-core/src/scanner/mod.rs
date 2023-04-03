use anyhow::Result;
use std::path::PathBuf;
use std::sync::mpsc::Sender;

pub mod local;
mod path_filter;

pub trait File {
    fn path(&self) -> &PathBuf;
    fn contents(&self) -> Result<String>;
}

pub trait FileScanner {
    type F: File;

    fn scan(&self, target: Sender<Self::F>) -> Result<()>;
}
