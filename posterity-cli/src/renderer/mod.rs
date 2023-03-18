pub mod markdown;
pub mod staging;

use anyhow::Result;
use crate::model::knowledge::KnowledgeTree;
use crate::renderer::staging::StagedFile;

pub trait Renderer {
    fn render(&self, root: &KnowledgeTree, out: &mut StagedFile) -> Result<()>;
}
