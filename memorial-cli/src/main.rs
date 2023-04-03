mod cli;

use crate::cli::app::App;
use anyhow::Result;

fn main() -> Result<()> {
    App::new()?.run()
}
