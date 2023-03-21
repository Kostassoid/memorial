use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use anyhow::{Context, Result};
use clap::{Arg, ArgAction, Command};
use crate::api::events::{Event, EventHandler};
use crate::cli::config::{Config, MarkdownOutput, Scanner};
use crate::collector::collector::Collector;
use crate::collector::file_matcher::FileTypeMatcher;
use crate::decorators::{Decorator, links, root};
use crate::model::attributes;
use crate::model::handle::Handle;
use crate::renderer::markdown::MarkdownRenderer;
use crate::scanner::local::LocalFileScanner;
use crate::parser::go::GoParser;
use crate::parser::rust::RustParser;
use crate::renderer::Renderer;
use crate::renderer::staging::StagingArea;
use crate::scanner::FileScanner;

pub struct App {
    config: Config,
    verbose_mode: bool,
}

//@[CLI]{title:CLI application}{do-not-collect}
impl App {
    pub fn new() -> Result<App> {
        /*@[CLI]
        The application is primarily designed to be run in non-interactive mode (e.g. as a pre-commit hook or during CI).
        Because of that reason and to emphasize using the code and VCS as much as possible (e.g. vs bash history),
        all of the parameters are embedded in a configuration file. With the only exception of verbose mode which can be
        used ad-hoc for figuring out some of the issues with collecting.
        */
        let args = Command::new("Memorial")
            .arg(
                Arg::new("config")
                    .help("A path to config file")
                    .short('c')
                    .default_value("memorial.toml")
                    .action(ArgAction::Set),
            )
            .arg(
                Arg::new("verbose")
                    .help("Verbose mode")
                    .short('v')
                    .action(ArgAction::SetTrue),
            )
            .get_matches();

        let config_path = args.get_one::<String>("config").unwrap();

        println!("Using config {}", &config_path);

        let config = Config::from_file(config_path)?;

        let verbose_mode = args.get_flag("verbose");

        Ok(App {
            config,
            verbose_mode,
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

        /*@[CLI/Rendering]
        Even though the overall design and the config model allow for using multiple renderers
        (even operating over the same collected notes), this feels like a rabbit hole of over-generalization.
        So the idea is to keep Markdown as the one and only renderer until the rest of the project is
        mature enough and there's a clear(er) vision of the roadmap.
        */

        collector.knowledge_mut().merge_attributes(&Handle::ROOT, HashMap::from([
            (attributes::OUTPUT_FILE_NAME.to_string(), self.config.output().markdown().path().to_string()),
            (attributes::TOC.to_string(), self.config.output().markdown().toc().to_string())
        ]));

        if self.verbose_mode {
            println!("\nCollected notes:\n{:#?}", collector.knowledge_mut());
        }

        println!("\nRendering into {}", self.config.output().markdown().path());

        renderer.render(
            collector.knowledge_mut(),
            &mut fs,
        )?;

        println!("\nFlushing the files...");

        let output_root = self.config.output().root().as_ref()
            .map(|r| PathBuf::from(r))
            .unwrap_or(env::current_dir()?);

        fs.flush_to_os_fs(output_root)?;

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
