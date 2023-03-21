pub mod markdown;
pub mod staging;

use std::path::Path;
use anyhow::Result;
use crate::model::knowledge::KnowledgeTree;
use crate::renderer::staging::StagingArea;

pub trait Renderer {
    fn render<P: AsRef<Path>>(&self, root: &KnowledgeTree, fs: &mut StagingArea, out: P) -> Result<()>;
}
