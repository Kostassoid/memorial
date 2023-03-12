use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use anyhow::{Result, anyhow};
use crate::collector::file_matcher::FileTypeMatcher;
use crate::collector::quote_parser::QuoteParser;
use crate::collector::QuoteSpan;
use crate::model::knowledge::KnowledgeTree;
use crate::model::note::{FileLocation, Note, NoteSpan};
use crate::parser::{FileParser, Quote};
use crate::scanner::{File, FileScanner};

pub struct Collector {
    knowledge: KnowledgeTree,
    parsers: HashMap<FileTypeMatcher, Box<dyn FileParser>>,
}

impl Collector {
    pub fn new() -> Collector {
        Collector{
            knowledge: KnowledgeTree::empty(),
            parsers: Default::default(),
        }
    }

    pub fn register_parser(&mut self, matcher: FileTypeMatcher, parser: Box<dyn FileParser>) {
        self.parsers.insert(matcher, parser);
    }

    pub fn scan<X: File>(&mut self, scanner: &dyn FileScanner<F=X>) -> Result<()> {
        let (tx, rx): (Sender<X>, Receiver<X>) = mpsc::channel();

        scanner.scan(tx)?;

        //todo: parallelize
        for f in rx {
            let path = f.path();
            let parser = self
                .find_parser(&path)
                .ok_or(anyhow!("Don't know how to parse {}", &path.display()))?;

            let quotes = parser.parse_from_str(&f.contents()?)?;

            let _errors:Vec<anyhow::Error> = quotes.into_iter()
                .filter_map(|q| { self.process_quote(q, path.clone()).err() })
                .collect();
        }

        Ok(())
    }

    fn process_quote(&mut self, quote: Quote, path: PathBuf) -> Result<()> {
        let mut parts = QuoteParser::parse_from_str(&quote.body)?;

        let handle = match parts.remove(0) {
            QuoteSpan::Link(h) => Some(h),
            _ => None
        }.unwrap(); //todo: handle more gracefully

        let mut attributes: HashMap<String, String> = Default::default();
        let mut note_spans: Vec<NoteSpan> = Default::default();

        for p in parts {
            match p {
                QuoteSpan::Attribute(k, v) => { attributes.insert(k, v); },
                QuoteSpan::Link(h) => note_spans.push(NoteSpan::Link(h)),
                QuoteSpan::Text(s) => note_spans.push(NoteSpan::Text(s)),
            }
        }

        let note = Note::new(
            FileLocation::new_relative(path, quote.line),
            note_spans,
        );

        self.knowledge.add_note(&handle, note);
        self.knowledge.merge_attributes(&handle, attributes);

        Ok(())
    }

    pub fn knowledge(&self) -> &KnowledgeTree {
        &self.knowledge
    }

    fn find_parser(&self, path: &PathBuf) -> Option<&Box<dyn FileParser>> {
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