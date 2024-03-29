use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

use anyhow::Result;
use pest::error::LineColLocation;

use crate::api::events::{Event, EventHandler};
use crate::collector::file_matcher::FileTypeMatcher;
use crate::collector::quote_parser::QuoteParser;
use crate::collector::{quote_parser, QuoteSpan};
use crate::model::attributes;
use crate::model::file_location::FileLocation;
use crate::model::note::{Note, NoteSpan};
use crate::model::tree::Node;
use crate::parser::{FileParser, Quote};
use crate::scanner::{File, FileScanner};

pub struct Collector {
    collected: Node,
    parsers: HashMap<FileTypeMatcher, Box<dyn FileParser>>,
}

impl Collector {
    pub fn new() -> Collector {
        Collector {
            collected: Node::root(),
            parsers: Default::default(),
        }
    }

    pub fn register_parser(&mut self, matcher: FileTypeMatcher, parser: Box<dyn FileParser>) {
        self.parsers.insert(matcher, parser);
    }

    pub fn scan<X: File>(
        &mut self,
        scanner: &dyn FileScanner<F = X>,
        event_handler: &mut dyn EventHandler,
    ) -> Result<()> {
        let (tx, rx): (Sender<X>, Receiver<X>) = mpsc::channel();

        event_handler.send(Event::ScanStarted)?;

        scanner.scan(tx)?;

        //todo: parallelize
        for f in rx {
            let path = f.path();

            event_handler.send(Event::ParsingStarted(path.clone()))?;

            let parser = match self.find_parser(&path) {
                Some(p) => p,
                _ => {
                    event_handler.send(Event::UnknownFileTypeEncountered(path.clone()))?;
                    continue;
                }
            };

            let quotes = parser.parse_from_str(&f.contents()?)?;

            let quotes_len = quotes.len();

            let errors: Vec<anyhow::Error> = quotes
                .into_iter()
                .filter_map(|q| self.process_quote(q, path.clone()).err())
                .filter(|e| {
                    //@[Core/Collector]: Ignoring parsing errors on collected quotes on (1,1) position to reduce false warnings.
                    match e.downcast_ref::<pest::error::Error<quote_parser::Rule>>() {
                        Some(ee) if ee.line_col == LineColLocation::Pos((1, 1)) => false,
                        _ => true,
                    }
                })
                .collect();

            for e in &errors {
                event_handler.send(Event::ParsingFailed(path.clone(), e.to_string()))?;
            }

            event_handler.send(Event::ParsingFinished(
                path.clone(),
                quotes_len - errors.len(),
            ))?;
        }

        event_handler.send(Event::ScanFinished)?;

        Ok(())
    }

    fn process_quote(&mut self, quote: Quote, path: PathBuf) -> Result<()> {
        let mut parts = QuoteParser::parse_from_str(&quote.body)?;

        let handle = match parts.remove(0) {
            QuoteSpan::Link(h) => Some(h),
            _ => None,
        }
        .unwrap(); //todo: handle more gracefully

        let mut attributes: HashMap<String, String> = Default::default();
        let mut note_spans: Vec<NoteSpan> = Default::default();

        for p in parts {
            match p {
                QuoteSpan::Attribute(k, v) => {
                    attributes.insert(k, v);
                }
                QuoteSpan::Link(h) => note_spans.push(NoteSpan::Link(h)),
                QuoteSpan::Text(s) => note_spans.push(NoteSpan::Text(s)),
            }
        }

        let note = Note::new(FileLocation::new_relative(path, quote.line), note_spans);

        if attributes.contains_key(attributes::DO_NOT_COLLECT) {
            attributes.remove(attributes::DO_NOT_COLLECT);
        } else {
            self.collected.add_note(&handle, note);
        }

        self.collected.merge_attributes(&handle, attributes);

        Ok(())
    }

    pub fn collected_mut(&mut self) -> &mut Node {
        &mut self.collected
    }

    fn find_parser(&self, path: &PathBuf) -> Option<&Box<dyn FileParser>> {
        // todo: find an efficient way
        self.parsers
            .iter()
            .find(|(k, _)| k.is_match(path))
            .map(|(_, v)| v)
    }
}

#[cfg(test)]
mod test {
    use crate::api::events::StubEventHandler;
    use crate::model::handle::Handle;
    use crate::parser::go::GoParser;

    use super::*;

    #[test]
    fn publishes_events_for_unknown_file_types() {
        let scanner = StubScanner {
            files: vec![StubFile {
                path: "path/to/file.xxx".into(),
                contents: "irrelevant".to_string(),
            }],
        };

        let mut event_handler = StubEventHandler::new();
        let mut collector = Collector::new();

        let result = collector.scan(&scanner, &mut event_handler);
        assert!(matches!(result, Ok(_)));

        let path: PathBuf = "path/to/file.xxx".into();
        assert_eq!(
            vec!(
                Event::ScanStarted,
                Event::ParsingStarted(path.clone()),
                Event::UnknownFileTypeEncountered(path.clone()),
                Event::ScanFinished,
            ),
            event_handler.events
        );
    }

    #[test]
    fn parses_and_extracts_quotes_from_files() {
        let scanner = StubScanner {
            files: vec![
                StubFile {
                    path: "path/to/file1.go".into(),
                    contents: "//@[a/b/c]: note 1".to_string(),
                },
                StubFile {
                    path: "path/to/file2.go".into(),
                    contents: "//@[a/b/c]{toggle}: note 2, see @[x/y/z] for more".to_string(),
                },
            ],
        };

        let mut event_handler = StubEventHandler::new();
        let mut collector = Collector::new();

        collector.register_parser(
            FileTypeMatcher::Extension("go".to_string()),
            Box::new(GoParser {}),
        );

        collector.scan(&scanner, &mut event_handler).unwrap();

        let path1: PathBuf = "path/to/file1.go".into();
        let path2: PathBuf = "path/to/file2.go".into();

        assert_eq!(
            vec!(
                Event::ScanStarted,
                Event::ParsingStarted(path1.clone()),
                Event::ParsingFinished(path1.clone(), 1),
                Event::ParsingStarted(path2.clone()),
                Event::ParsingFinished(path2.clone(), 1),
                Event::ScanFinished,
            ),
            event_handler.events
        );

        let node = collector
            .collected
            .find_node(&Handle::from_str("a/b/c").unwrap())
            .unwrap();

        assert_eq!(
            vec!(
                Note::new(
                    FileLocation::new_relative("path/to/file1.go", 1),
                    vec!(NoteSpan::Text("note 1".to_string())),
                ),
                Note::new(
                    FileLocation::new_relative("path/to/file2.go", 1),
                    vec!(
                        NoteSpan::Text("note 2, see".to_string()),
                        NoteSpan::Link(Handle::from_str("x/y/z").unwrap()),
                        NoteSpan::Text("for more".to_string()),
                    ),
                ),
            ),
            *node.notes()
        );

        assert_eq!(
            HashMap::from([("toggle".to_string(), "".to_string())]),
            *node.attributes()
        )
    }

    #[derive(Clone)]
    struct StubFile {
        path: PathBuf,
        contents: String,
    }

    impl File for StubFile {
        fn path(&self) -> &PathBuf {
            &self.path
        }

        fn contents(&self) -> Result<String> {
            Ok(self.contents.clone())
        }
    }

    struct StubScanner {
        files: Vec<StubFile>,
    }

    impl FileScanner for StubScanner {
        type F = StubFile;

        fn scan(&self, target: Sender<Self::F>) -> Result<()> {
            for f in &self.files {
                target.send(f.clone())?;
            }
            Ok(())
        }
    }
}
