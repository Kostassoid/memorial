mod model;
mod scanner;
mod parser;
mod collector;
mod renderer;
mod api;
mod cli;

use anyhow::Result;
use crate::cli::app::App;

fn main() -> Result<()> {
    App::new()?.run()
}
