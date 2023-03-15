use std::env;
use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use anyhow::Result;
use clap::{Arg, ArgAction, Command};
use crate::api::events::Event;
use crate::cli::config::{Config, MarkdownOutput};
use crate::collector::collector::Collector;
use crate::collector::file_matcher::FileTypeMatcher;
use crate::renderer::markdown::MarkdownRenderer;
use crate::scanner::local::LocalFileScanner;
use crate::parser::go::GoParser;
use crate::renderer::Renderer;

pub struct App {
    config: Config
}

impl App {
    pub fn new() -> Result<App> {
        let args = Command::new("Posterity")
            .arg(
                Arg::new("config")
                    .help("A path to config file")
                    .short('c')
                    .default_value("posterity.toml")
                    .action(ArgAction::Set),
            )
            .get_matches();

        let config_path = args.get_one::<String>("config").unwrap();

        println!("Using config {}", &config_path);

        let config = Config::from_file(config_path)?;

        // println!("Using config: {:#?}", config);

        Ok(App {
            config
        })
    }

    pub fn run(self) -> Result<()> {
        let scanner_config = self.config.scanner().local();
        let scanner = LocalFileScanner::new(
            scanner_config.root().as_ref()
                .map(|r| PathBuf::from(r))
                .unwrap_or(env::current_dir()?),
            scanner_config.include().clone(),
            scanner_config.exclude().as_ref()
                .map(|v| v.clone())
                .unwrap_or(vec![]),
        ).unwrap();

        let mut collector = Collector::new(
            self.config.scanner().skip_unknown_files().unwrap_or(true)
        );
        collector.register_parser(FileTypeMatcher::Extension("go".to_string()), Box::new(GoParser {}));

        let renderer = MarkdownRenderer::new();

        let (tx, rx): (Sender<Event>, Receiver<Event>) = mpsc::channel();

        collector.scan(&scanner, tx)?;

        for e in rx {
            println!("E: {:?}", e);
        }

        let out = renderer.render(collector.knowledge_mut())?;
        println!("{}", out);

        Ok(())
    }
}