pub mod meta;
pub mod links;

use anyhow::Result;
use crate::model::knowledge::KnowledgeTree;

pub trait Decorator {
    fn decorate(&self, tree: &mut KnowledgeTree) -> Result<()>;
}
