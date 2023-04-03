mod cli;

use anyhow::Result;
use crate::cli::app::App;

fn main() -> Result<()> {
    App::new()?.run()
}
