use std::env;
use std::io::stdout;
use std::path::PathBuf;
use anyhow::{Context, Result};
use clap::{Arg, ArgAction, Command};
use crate::api::events::{Event, EventHandler};
use crate::cli::config::{Config, MarkdownOutput};
use crate::collector::collector::Collector;
use crate::collector::file_matcher::FileTypeMatcher;
use crate::renderer::markdown::MarkdownRenderer;
use crate::scanner::local::LocalFileScanner;
use crate::parser::go::GoParser;
use crate::parser::rust::RustParser;
use crate::renderer::Renderer;
use crate::renderer::staging_fs::StagingFS;

pub struct App {
    config: Config
}

//@[CLI]{title:CLI application}{do-not-collect}
impl App {
    pub fn new() -> Result<App> {
        /*@[CLI]
        The application is primarily designed to be run automatically (e.g. as a pre-commit hook or during CI).
        Because of that reason and to emphasize using the code and VCS as much as possible (e.g. vs bash history),
        the only argument for the CLI application for now is just a path to the configuration. Which is
        also optional.
        */
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

        Ok(App {
            config
        })
    }

    pub fn run(mut self) -> Result<()> {
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
        collector.register_parser(FileTypeMatcher::Extension("rs".to_string()), Box::new(RustParser {}));

        let mut fs = StagingFS::new();

        let renderer = MarkdownRenderer::new();

        collector.scan(&scanner, &mut self)?;

        println!("Writing to {}", self.config.output().markdown().path());

        let mut out = fs.open_as_new(self.config.output().markdown().path());

        renderer.render(collector.knowledge_mut(), &mut out)?;

        fs.flush_to_os_fs(env::current_dir()?)?;

        println!("Done!");

        Ok(())
    }
}

impl EventHandler for App {
    fn send(&mut self, event: Event) -> Result<()> {
        match event {
            Event::ScanStarted => println!("Started scanning..."),
            Event::ParsingStarted(p) => println!("Parsing {}", p.to_str().unwrap()),
            Event::ParsingWarning(msg) => println!("- Warning: {}", msg),
        }
        Ok(())
    }
}
