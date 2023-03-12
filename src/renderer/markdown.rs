use std::ops::Add;
use crate::model::handle::HandlePart;
use crate::model::knowledge::KnowledgeTree;
use crate::renderer::Renderer;

struct MarkdownRenderer {
    title: String,
    link_prefix: String,
}

impl MarkdownRenderer {
    fn add_section(&self, level: usize, node: &KnowledgeTree) {
        node.children()
            .iter()
            .for_each(|(h, n)| {
                //rendered.add(&format!("{} {}", "#".repeat(level), h));

                // todo: this

                self.add_section(level + 1, n)
            });
    }
}

impl Renderer for MarkdownRenderer {
    fn render(&self, root: &KnowledgeTree) -> String {
        let mut rendered = String::new();

        rendered = rendered.add(&format!("# {}", self.title));

        self.add_section(2, root);

        rendered
    }
}

