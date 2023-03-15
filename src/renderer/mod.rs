pub mod markdown;
pub mod staging_fs;

use anyhow::Result;
use crate::model::knowledge::KnowledgeTree;
use crate::renderer::staging_fs::{StagingFile, StagingFS};

pub trait Renderer {
    fn render(&self, root: &KnowledgeTree, out: &mut StagingFile) -> Result<()>;
}
