mod markdown;

use crate::model::knowledge::KnowledgeTree;

pub trait Renderer {
    fn render(&self, root: &KnowledgeTree) -> String;
}
