use crate::model::knowledge::KnowledgeTree;

pub trait Renderer {
    fn render(root: KnowledgeTree) -> String;
}

struct MarkdownRenderer {}

impl Renderer for MarkdownRenderer {
    fn render(root: KnowledgeTree) -> String {
        todo!()
    }
}