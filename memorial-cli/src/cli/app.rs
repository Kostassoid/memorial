use anyhow::Result;
use clap::{Arg, ArgAction, Command};

use crate::cli::config::Config;
use crate::cli::scan::ScanAction;
use crate::cli::Action;

pub struct App {}

//@[CLI]{title:CLI application}{do-not-collect}
impl App {
    pub fn setup() -> Result<impl Action> {
        /*@[CLI]:
        The application is primarily designed to be run in non-interactive mode (e.g. as a pre-commit
        hook or during CI). Because of that reason and to emphasize using VCS for anything important,
        practically all parameters are read from a configuration file instead of command-line arguments.
        */
        let args = Command::new("Memorial")
            .subcommand_required(true)
            .subcommand(
                /*@[CLI]: `scan` command is the only one implemented so far but it's not made a default
                because of the likely future extensions.
                */
                Command::new("scan")
                    .about("Scans source files and generates documentation files from found notes.")
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
                    ),
            );

        match args
            .try_get_matches()
            .unwrap_or_else(|e| e.exit())
            .subcommand()
        {
            Some(("scan", scan_args)) => {
                let config_path = scan_args.get_one::<String>("config").unwrap();
                let config = Config::from_file(config_path)?;
                let verbose_mode = scan_args.get_flag("verbose");

                println!("Using config file {}", &config_path);

                Ok(ScanAction {
                    config,
                    verbose_mode,
                })
            }
            _ => unreachable!(),
        }
    }
}
