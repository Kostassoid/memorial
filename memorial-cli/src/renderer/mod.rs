pub mod markdown;
pub mod staging;

use anyhow::Result;
use crate::model::knowledge::KnowledgeTree;
use crate::renderer::staging::StagingArea;

pub trait Renderer {
    fn render(&self, root: &KnowledgeTree, fs: &mut StagingArea) -> Result<()>;
}
