use std::fmt::Write;

use anyhow::Result;

use crate::model::attributes;
use crate::model::file_location::FileLocation;
use crate::model::handle::Handle;
use crate::model::note::{Note, NoteSpan};
use crate::model::tree::Node;
use crate::renderer::staging::{StagedFile, StagingArea};
use crate::renderer::Renderer;

pub struct MarkdownRenderer {}

struct RendererSession<'a> {
    root: &'a Node,
    out: &'a mut StagedFile,
}

impl MarkdownRenderer {
    pub fn new() -> MarkdownRenderer {
        MarkdownRenderer {}
    }
}

impl Renderer for MarkdownRenderer {
    fn render(&self, root: &Node, fs: &mut StagingArea) -> Result<()> {
        /*@[Core/Renderer/Markdown]:
        One possible future improvement is allowing to render the collected notes into multiple files.
        This can be user-controlled by using attributes. Hence that's how output file path is
        passed to the renderer. But, for now, only a value from the root node is used.
        */
        RendererSession {
            root,
            out: fs.open_as_new(root.attributes().get(attributes::OUTPUT_FILE_NAME).unwrap()),
        }
        .render()
    }
}

/*@[Core/Renderer/Markdown]:
The renderer is currently implemented using low level string builders. This seemed like a good idea
during the initial development phase as integrating with template engine libraries would require
preparing the data in a certain way, which, depending on the engine implementation could limit the
features exposed from the `Renderer`.
*/
impl<'a> RendererSession<'a> {
    fn render(mut self) -> Result<()> {
        self.render_node(1, self.root)?;

        self.render_footer()?;

        Ok(())
    }

    fn w<'b, S: Into<&'b str>>(&mut self, s: S) -> Result<()> {
        self.out.write_str(s.into())?; // anyhow vs fmt::Result weirdness
        Ok(())
    }

    fn render_footer(&mut self) -> Result<()> {
        self.w(&*format!(
            "\n---\n<sub>Generated by [Memorial](https://github.com/Kostassoid/memorial) v{} at _{}_.</sub>",
            &self.root.attributes().get(attributes::APP_VERSION).unwrap_or(&"?".to_string()),
            &self.root.attributes().get(attributes::TIMESTAMP).unwrap_or(&"?".to_string()),
        ))
    }

    fn render_node(&mut self, level: usize, node: &Node) -> Result<()> {
        self.w(&*format!(
            "{} <a id=\"{}\"></a> {}\n\n",
            "#".repeat(level),
            node.handle().as_url_safe_string(),
            self.resolve_node_title(node.handle()),
        ))?;

        self.render_toc(level + 1, node)?;

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

    fn render_toc(&mut self, level: usize, node: &Node) -> Result<()> {
        if node
            .attributes()
            .get(attributes::TOC)
            .unwrap_or(&"false".to_string())
            != "true"
        {
            return Ok(());
        }

        self.w(&*format!("{} Table of contents\n\n", "#".repeat(level),))?;

        self.render_toc_links(0, node)?;

        self.w("\n")?;

        Ok(())
    }

    fn render_toc_links(&mut self, level: usize, node: &Node) -> Result<()> {
        if level > 0 {
            self.w(&*format!(
                "{}- {}\n",
                "\t".repeat(level - 1),
                self.format_link(node.handle())
            ))?;
        }

        for (_, n) in node.children() {
            self.render_toc_links(level + 1, n)?;
        }

        Ok(())
    }

    fn format_note(&self, note: &Note) -> String {
        let mut formatted = String::from("> ");

        for s in note.spans() {
            match s {
                NoteSpan::Text(s) => formatted
                    .write_str(&format!("{} ", s.replace("\n", "\n> ")))
                    .unwrap(),
                NoteSpan::Link(handle) => formatted.write_str(&self.format_link(handle)).unwrap(),
            }
        }

        let l = note.location();
        formatted
            .write_str(&format!("\n\nat {}\n", Self::format_location(l)))
            .unwrap();

        formatted.write_str("\n\n").unwrap();

        formatted
    }

    fn format_location(l: &FileLocation) -> String {
        format!("[{} (line {})]({})\n", l.title(), l.line(), l.path())
    }

    fn format_link(&self, h: &Handle) -> String {
        format!(
            "[{}](#{}) ",
            self.resolve_node_title(h),
            h.as_url_safe_string(),
        )
    }

    fn resolve_node_title(&self, handle: &Handle) -> String {
        self.root
            .find_node(handle)
            .map(|n| n.attributes().get(attributes::TITLE))
            .flatten()
            .map(|s| s.to_string())
            .unwrap_or(
                handle
                    .parts()
                    .last()
                    .unwrap_or(&String::from("(root)"))
                    .to_string(),
            )
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::model::handle::Handle;

    use super::*;

    #[test]
    fn render_from_tree() {
        let mut tree = Node::root();

        tree.merge_attributes(
            &Handle::ROOT,
            HashMap::from([
                (attributes::TITLE.to_string(), "Big Nice Title".to_string()),
                (attributes::APP_VERSION.to_string(), "0.1.0".to_string()),
                (
                    attributes::TIMESTAMP.to_string(),
                    "2023-03-01 12:34:56".to_string(),
                ),
                (attributes::OUTPUT_FILE_NAME.to_string(), "test".to_string()),
                (attributes::TOC.to_string(), "true".to_string()),
            ]),
        );

        tree.add_note(
            &Handle::from_str("a/b/c").unwrap(),
            Note::new(
                FileLocation::new_relative("path/to/file1.ext", 123),
                vec![
                    NoteSpan::Text("note 1".to_string()),
                    NoteSpan::Link(Handle::from_str("a/b/d").unwrap()),
                ],
            ),
        );

        tree.merge_attributes(
            &Handle::from_str("a/b/c").unwrap(),
            HashMap::from([(attributes::TITLE.to_string(), "Sub title".to_string())]),
        );

        tree.add_note(
            &Handle::from_str("a/b/d").unwrap(),
            Note::new(
                FileLocation::new_relative("path/to/file2.ext", 234),
                vec![NoteSpan::Text("note 2".to_string())],
            ),
        );

        let mut fs = StagingArea::new();
        let renderer = MarkdownRenderer::new();

        renderer.render(&tree, &mut fs).unwrap();

        let generated = String::from_utf8(fs.open("test").unwrap().contents().clone())
            .unwrap()
            .replace(" \n", "\n");

        let expected = r#"
# <a id=""></a> Big Nice Title

## Table of contents

- [a](#a)
	- [b](#a+b)
		- [Sub title](#a+b+c)
		- [d](#a+b+d)

## <a id="a"></a> a

### <a id="a+b"></a> b

#### <a id="a+b+c"></a> Sub title

> note 1 [d](#a+b+d)

at [path/to/file1.ext (line 123)](path/to/file1.ext)



#### <a id="a+b+d"></a> d

> note 2

at [path/to/file2.ext (line 234)](path/to/file2.ext)




_Mentioned in:_
- [Sub title](#a+b+c)

---
<sub>Generated by [Memorial](https://github.com/Kostassoid/memorial) v0.1.0 at _2023-03-01 12:34:56_.</sub>
        "#.trim();

        assert_eq!(expected, generated);
    }
}
