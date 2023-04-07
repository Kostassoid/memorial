use anyhow::Result;

use crate::cli::app::App;
use crate::cli::Action;

mod cli;

fn main() -> Result<()> {
    App::setup()?.run()
}
