use std::path::Path;
use anyhow::{anyhow, Context, Result};
use serde_derive::{Deserialize};
use derive_getters::{Getters, Dissolve};

use crate::scanner;

#[derive(Deserialize, Debug, Getters)]
pub struct Config {
    title: String,
    scanner: Scanner,
    processor: Processor,
}

#[derive(Deserialize, Debug, Getters)]
pub struct Scanner {
    local: LocalScanner,
    #[serde(alias = "skip-unknown-files")]
    skip_unknown_files: Option<bool>,
}

#[derive(Deserialize, Debug, Getters)]
pub struct LocalScanner {
    root: Option<String>,
    include: Vec<String>,
    exclude: Option<Vec<String>>,
}

#[derive(Deserialize, Debug, Getters)]
pub struct Processor {
    links: Option<LinksProcessor>,
}

#[derive(Deserialize, Debug, Getters)]
pub struct LinksProcessor {
    target: String,
}

#[derive(Deserialize, Debug, Getters)]
pub struct Output {
    markdown: MarkdownOutput,
}

#[derive(Deserialize, Debug, Getters)]
pub struct MarkdownOutput {
    path: String,
    toc: bool,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Config> {
        let raw = std::fs::read_to_string(path.as_ref()).context(format!(
            "Unable to load config file from {}",
            path.as_ref()
                .to_str()
                .ok_or(anyhow!("Invalid config path"))?
        ))?;

        Self::from_str(&raw)
    }

    pub fn from_str(raw: &str) -> Result<Self> {
        toml::from_str(raw).context("Unable to parse config file")
    }
}