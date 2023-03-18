use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use anyhow::{Result, anyhow};
use pest::error::LineColLocation;
use crate::api::events::{Event, EventHandler};
use crate::collector::file_matcher::FileTypeMatcher;
use crate::collector::quote_parser::QuoteParser;
use crate::collector::{quote_parser, QuoteSpan};
use crate::model::attributes;
use crate::model::file_location::FileLocation;
use crate::model::knowledge::KnowledgeTree;
use crate::model::note::{Note, NoteSpan};
use crate::parser::{FileParser, Quote};
use crate::scanner::{File, FileScanner};

pub struct Collector {
    skip_unknown_files: bool,
    knowledge: KnowledgeTree,
    parsers: HashMap<FileTypeMatcher, Box<dyn FileParser>>,
}

impl Collector {
    pub fn new(skip_unknown_files: bool) -> Collector {
        Collector{
            skip_unknown_files,
            knowledge: KnowledgeTree::empty(),
            parsers: Default::default(),
        }
    }

    pub fn register_parser(&mut self, matcher: FileTypeMatcher, parser: Box<dyn FileParser>) {
        self.parsers.insert(matcher, parser);
    }

    pub fn scan<X: File>(&mut self, scanner: &dyn FileScanner<F=X>, event_handler: &mut dyn EventHandler) -> Result<()> {
        let (tx, rx): (Sender<X>, Receiver<X>) = mpsc::channel();

        event_handler.send(Event::ScanStarted)?;

        scanner.scan(tx)?;

        //todo: parallelize
        for f in rx {
            let path = f.path();

            event_handler.send(Event::ParsingStarted(path.clone()))?;

            let parser = match self
                .find_parser(&path) {
                Some(p) => p,
                _ if self.skip_unknown_files => {
                    event_handler.send(Event::ParsingWarning("Unknown file type".to_string()))?;
                    continue;
                },
                _ => return Err(anyhow!("Unknown file type {}", &path.display()))
            };

            let quotes = parser.parse_from_str(&f.contents()?)?;

            let quotes_len = quotes.len();

            let errors: Vec<anyhow::Error> = quotes.into_iter()
                .filter_map(|q| { self.process_quote(q, path.clone()).err() })
                .collect();

            event_handler.send(Event::ParsingFinished(quotes_len - errors.len()))?;

            //@[Core/Collector] Ignoring parsing errors on collected quotes on (1,1) position to reduce false warnings.
            errors.iter()
                .filter(|e| {
                    match e.downcast_ref::<pest::error::Error<quote_parser::Rule>>() {
                        Some(ee) if ee.line_col == LineColLocation::Pos((1, 1)) => false,
                        _ => true
                    }
                })
                .map(|e| event_handler.send(Event::ParsingWarning(e.to_string())))
                .collect::<Vec<Result<()>>>();
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

        if attributes.contains_key(attributes::DO_NOT_COLLECT) {
            attributes.remove(attributes::DO_NOT_COLLECT);
        } else {
            self.knowledge.add_note(&handle, note);
        }

        self.knowledge.merge_attributes(&handle, attributes);

        Ok(())
    }

    pub fn knowledge_mut(&mut self) -> &mut KnowledgeTree {
        &mut self.knowledge
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
    use crate::api::events::StubEventHandler;
    use super::*;
    use crate::scanner::local::LocalFileScanner;
    use crate::parser::go::GoParser;

    // todo: don't have to use real files
    #[test]
    fn collect_from_local_files() {
        let mut collector = Collector::new(false);
        collector.register_parser(FileTypeMatcher::Extension("go".to_string()), Box::new(GoParser{}));

        let scanner = LocalFileScanner::new(
            env::current_dir().unwrap(),
            vec!("src/tests/**/*.go".into()),
            vec!("**/*bad*".into()),
        ).unwrap();

        let mut event_handler = StubEventHandler{ events: vec![] };

        collector.scan(&scanner, &mut event_handler).unwrap();

        let knowledge = collector.knowledge_mut();
        println!("k = {knowledge:#?}");
    }
}