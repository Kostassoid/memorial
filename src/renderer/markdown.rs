use std::fmt::Write;
use std::time::SystemTime;
use anyhow::{Context, Result};
use time::OffsetDateTime;
use crate::model::attributes;
use crate::model::file_location::FileLocation;
use crate::model::handle::{Handle, HandlePart};
use crate::model::knowledge::KnowledgeTree;
use crate::model::note::{Note, NoteSpan};
use crate::renderer::Renderer;

pub struct MarkdownRenderer {
}

struct RendererSession<'a> {
    root: &'a KnowledgeTree,
    rendered: String,
}

impl MarkdownRenderer {
    pub fn new() -> MarkdownRenderer {
        MarkdownRenderer{}
    }
}

impl <'a> RendererSession<'a> {
    fn render(mut self) -> Result<String> {
        self.render_node(1, self.root, )?;

        self.render_footer()?;

        Ok(self.rendered)
    }

    fn w<'b, S: Into<&'b str>>(&mut self, s: S) -> Result<()> {
        self.rendered.write_str(s.into())?; // anyhow vs fmt::Result weirdness
        Ok(())
    }

    fn render_footer(&mut self) -> Result<()> {
        self.w(&*format!(
            "\n---\n<sub>Generated by [Posterity](https://github.com/Kostassoid/posterity) {} at _{}_.</sub>",
            &self.root.attributes().get(attributes::APP_VERSION).unwrap_or(&"v?".to_string()),
            OffsetDateTime::from(SystemTime::now())
                .format(&time::format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap())
                .unwrap(),
        ))
    }

    fn render_node(&mut self, level: usize, node: &KnowledgeTree) -> Result<()> {
        self.w(&*format!(
            "{} <a id=\"{}\"></a> {}\n\n",
            "#".repeat(level),
            node.handle().as_url_safe_string(),
            self.resolve_node_title(node.handle()),
        ))?;

        for n in node.notes() {
            self.w(&*self.format_note(n))?;
        }

        if !node.extra().is_empty() {
            self.w("\n_Extra references:_\n")?;
            for l in node.extra() {
                self.w(&*format!("- {}\n", Self::format_location(l)))?;
            }
        }

        if !node.mentions().is_empty() {
            self.w("\n_Mentioned in:_\n")?;
            for m in node.mentions() {
                self.w(&*format!("- {}\n", self.format_link(m)))?;
            }
        }

        for (_, n) in node.children() {
            self.render_node(level + 1, n)?;
        }

        Ok(())
    }

    fn format_note(&self, note: &Note) -> String {
        let mut formatted = String::from("> ");

        for s in note.spans() {
            match s {
                NoteSpan::Text(s) => {
                    formatted.write_str(&format!("{} ", s.replace("\n", "\n> "))).unwrap()
                },
                NoteSpan::Link(handle) => {
                    formatted.write_str(&self.format_link(handle)).unwrap()
                }
            }
        }

        let l = note.location();
        formatted.write_str(&format!("\n\nat {}\n", Self::format_location(l))).unwrap();

        formatted.write_str("\n\n").unwrap();

        formatted
    }

    fn format_location(l: &FileLocation) -> String {
        format!("[{} (line {})]({})\n", l.path(), l.line(), l.path())
    }

    fn format_link(&self, h: &Handle) -> String {
        format!(
            "[{}](#{}) ",
            self.resolve_node_title(h),
            h.as_url_safe_string(),
        )
    }

    fn resolve_node_title(&self, handle: &Handle) -> String {
        self.root.find_node(handle)
            .map(|n| n.attributes().get(attributes::ALIAS))
            .flatten()
            .map(|s| s.to_string())
            .unwrap_or(handle.parts().last().unwrap_or(&String::from("(root)")).to_string())
    }
}

impl Renderer for MarkdownRenderer {
    fn render(&self, root: &KnowledgeTree) -> Result<String> {
        RendererSession {
            root,
            rendered: String::new(),
        }.render()
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::env;
    use crate::collector::collector::Collector;
    use crate::collector::file_matcher::FileTypeMatcher;
    use crate::model::handle::Handle;
    use super::*;
    use crate::scanner::local::{LocalConfig, LocalFileScanner};
    use crate::parser::go::GoParser;

    // todo: don't have to use real files
    #[test]
    fn render_from_local_files() {
        let config = LocalConfig::new(
            env::current_dir().unwrap(),
            vec!("src/tests/**/*.go".into()),
            vec!("**/*bad*".into()),
        );

        let mut collector = Collector::new();
        collector.register_parser(FileTypeMatcher::Extension("go".to_string()), Box::new(GoParser {}));

        let scanner = LocalFileScanner::new(config).unwrap();
        collector.scan(&scanner).unwrap();

        let knowledge = collector.knowledge_mut();

        knowledge.merge_attributes(
            &Handle::ROOT,
            HashMap::from([
                (attributes::ALIAS.to_string(), "Big Nice Title".to_string()),
                (attributes::APP_VERSION.to_string(), "v0.1.0".to_string()),
            ]),
        );

        let renderer = MarkdownRenderer::new();

        let rendered = renderer.render(&knowledge).unwrap();
        println!("{}", rendered);
    }
}