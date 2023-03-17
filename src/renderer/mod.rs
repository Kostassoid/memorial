pub mod markdown;
pub mod staging_fs;

use anyhow::Result;
use crate::model::knowledge::KnowledgeTree;
use crate::renderer::staging_fs::StagingFile;

pub trait Renderer {
    fn render(&self, root: &KnowledgeTree, out: &mut StagingFile) -> Result<()>;
}
