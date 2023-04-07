use anyhow::Result;

use crate::model::tree::Node;
use crate::renderer::staging::StagingArea;

pub mod markdown;
pub mod staging;

pub trait Renderer {
    fn render(&self, root: &Node, fs: &mut StagingArea) -> Result<()>;
}
