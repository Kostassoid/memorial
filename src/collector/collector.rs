use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use anyhow::{Result, anyhow, Context};
use crate::collector::file_matcher::FileTypeMatcher;
use crate::model::handle::Handle;
use crate::model::knowledge::KnowledgeTree;
use crate::model::note::{FileLocation, Note};
use crate::parser::QuoteParser;
use crate::scanner::{File, FileScanner};

pub struct Collector {
    knowledge: KnowledgeTree,
    parsers: HashMap<FileTypeMatcher, Box<dyn QuoteParser>>,
}

impl Collector {
    pub fn new() -> Collector {
        Collector{
            knowledge: KnowledgeTree::empty(),
            parsers: Default::default(),
        }
    }

    pub fn register_parser(&mut self, matcher: FileTypeMatcher, parser: Box<dyn QuoteParser>) {
        self.parsers.insert(matcher, parser);
    }

    pub fn scan<X: File>(&mut self, scanner: &dyn FileScanner<F=X>) -> Result<()> {
        let (tx, rx): (Sender<X>, Receiver<X>) = mpsc::channel();

        scanner.scan(tx)?;

        //todo: parallelize
        for f in rx {
            let path = &f.path();
            let parser = self
                .find_parser(path)
                .ok_or(anyhow!("Don't know how to parse {}", path.display()))?;

            let quotes = parser.extract_from_str(&f.contents()?)?;
            quotes.into_iter().for_each(|q| {
                let handle = Handle::build_from("fake/path").unwrap(); //todo: this
                let note = Note::new(
                    FileLocation::new(path, q.line),
                    Some(q.body),
                    vec![],
                );
                self.knowledge.add(handle, note);
            })
        }

        Ok(())
    }

    pub fn knowledge(&self) -> &KnowledgeTree {
        &self.knowledge
    }

    fn find_parser(&self, path: &PathBuf) -> Option<&Box<dyn QuoteParser>> {
        // todo: find an efficient way
        self.parsers
            .iter()
            .find(|(k, _)| { k.is_match(path)})
            .map(|(_, v)| v)
    }
}

#[cfg(test)]
mod test {
    use std::env;
    use super::*;
    use crate::scanner::local::{LocalConfig, LocalFileScanner};
    use crate::parser::go::GoParser;

    // todo: don't have to use real files
    #[test]
    fn collect_from_local_files() {
        let config = LocalConfig::new(
            env::current_dir().unwrap(),
            vec!("src/tests/**/*.go".into()),
            vec!("**/*bad*".into()),
        );

        let mut collector = Collector::new();
        collector.register_parser(FileTypeMatcher::Extension("go".to_string()), Box::new(GoParser{}));

        let scanner = LocalFileScanner::new(config).unwrap();
        collector.scan(&scanner).unwrap();

        let knowledge = collector.knowledge();
        println!("k = {knowledge:#?}");
    }
}