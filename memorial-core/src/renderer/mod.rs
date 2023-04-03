pub mod markdown;
pub mod staging;

use crate::model::knowledge::KnowledgeTree;
use crate::renderer::staging::StagingArea;
use anyhow::Result;

pub trait Renderer {
    fn render(&self, root: &KnowledgeTree, fs: &mut StagingArea) -> Result<()>;
}
