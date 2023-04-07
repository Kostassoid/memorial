use anyhow::Result;

use crate::model::tree::Node;

pub mod links;
pub mod root;

pub trait Decorator {
    fn decorate(&self, tree: &mut Node) -> Result<()>;
}
