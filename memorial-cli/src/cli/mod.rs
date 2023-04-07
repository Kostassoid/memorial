use anyhow::Result;

pub mod app;
mod config;
mod scan;

pub trait Action {
    fn run(&mut self) -> Result<()>;
}
