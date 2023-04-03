pub mod links;
pub mod root;

use crate::model::knowledge::KnowledgeTree;
use anyhow::Result;

pub trait Decorator {
    fn decorate(&self, tree: &mut KnowledgeTree) -> Result<()>;
}
