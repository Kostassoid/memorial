use crate::model::knowledge::KnowledgeNode;

pub trait Renderer {
    fn render(root: KnowledgeNode) -> String;
}

struct MarkdownRenderer {}

impl Renderer for MarkdownRenderer {
    fn render(root: KnowledgeNode) -> String {
        todo!()
    }
}