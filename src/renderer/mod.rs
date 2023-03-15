pub mod markdown;

use anyhow::Result;
use crate::model::knowledge::KnowledgeTree;

pub trait Renderer {
    fn render(&self, root: &KnowledgeTree) -> Result<String>;
}
