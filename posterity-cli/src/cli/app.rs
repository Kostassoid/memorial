use std::env;
use std::path::PathBuf;
use anyhow::{Context, Result};
use clap::{Arg, ArgAction, Command};
use crate::api::events::{Event, EventHandler};
use crate::cli::config::{Config, MarkdownOutput, Scanner};
use crate::collector::collector::Collector;
use crate::collector::file_matcher::FileTypeMatcher;
use crate::decorators::{Decorator, links, root};
use crate::renderer::markdown::MarkdownRenderer;
use crate::scanner::local::LocalFileScanner;
use crate::parser::go::GoParser;
use crate::parser::rust::RustParser;
use crate::renderer::Renderer;
use crate::renderer::staging::StagingArea;
use crate::scanner::FileScanner;

pub struct App {
    config: Config,
}

//@[CLI]{title:CLI application}{do-not-collect}
impl App {
    pub fn new() -> Result<App> {
        /*@[CLI]
        The application is primarily designed to be run automatically (e.g. as a pre-commit hook or during CI).
        Because of that reason and to emphasize using the code and VCS as much as possible (e.g. vs bash history),
        the only argument for the CLI application for now is just a path to the configuration file.
        Which is also optional.
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
        let scanner = self.build_scanner()?;

        let mut collector = self.build_collector()?;

        let decorators = self.build_decorators()?;

        let mut fs = StagingArea::new();

        let renderer = MarkdownRenderer::new();

        collector.scan(&scanner, &mut self)?;

        if collector.knowledge_mut().is_empty() {
            println!("No notes found. Stopping here.");
            return Ok(())
        }

        decorators.iter().for_each(|d| d.decorate(collector.knowledge_mut()).unwrap());

        println!("\nRendering into {}", self.config.output().markdown().path());

        renderer.render(collector.knowledge_mut(), &mut fs,self.config.output().markdown().path())?;

        println!("\nFlushing the files...");

        fs.flush_to_os_fs(env::current_dir()?)?;

        println!("Done!");

        Ok(())
    }

    fn build_scanner(&self) -> Result<impl FileScanner> {
        let scanner_config = self.config.scanner().local();

        let root = scanner_config.root().as_ref()
            .map(|r| PathBuf::from(r))
            .unwrap_or(env::current_dir()?);

        Ok(LocalFileScanner::new(
            root,
            scanner_config.include().clone(),
            scanner_config.exclude().as_ref()
                .map(|v| v.clone())
                .unwrap_or(vec![]),
        )?)
    }

    fn build_collector(&self) -> Result<Collector> {
        let mut collector = Collector::new(
            self.config.scanner().skip_unknown_files().unwrap_or(true)
        );
        collector.register_parser(FileTypeMatcher::Extension("go".to_string()), Box::new(GoParser {}));
        collector.register_parser(FileTypeMatcher::Extension("rs".to_string()), Box::new(RustParser {}));

        Ok(collector)
    }

    fn build_decorators(&self) -> Result<Vec<Box<dyn Decorator>>> {
        let mut decorators: Vec<Box<dyn Decorator>> = vec!(
            Box::new(root::RootDecorator {
                title: self.config.title().clone(),
            })
        );

        if let Some(l) = self.config.decorators().external_links() {
            decorators.push(Box::new(links::LinksDecorator::new(
                l.root().to_string(),
                l.format().clone(),
            )?));
        };

        Ok(decorators)
    }
}

impl EventHandler for App {
    fn send(&mut self, event: Event) -> Result<()> {
        match event {
            Event::ScanStarted => println!("Started scanning..."),
            Event::ParsingStarted(p) => println!("Parsing {}", p.to_str().unwrap()),
            Event::ParsingFinished(_, notes) if notes > 0 => println!("- Found {} note(s)", notes),
            Event::ParsingFinished(_, _) => { },
            Event::ParsingWarning(_, msg) => println!("- Warning: {}", msg),
            Event::ScanFinished => { },
        }
        Ok(())
    }
}
