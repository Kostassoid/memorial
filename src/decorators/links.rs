use std::fmt::Debug;
use url::Url;
use anyhow::Result;
use crate::decorators::Decorator;
use crate::model::file_location::{FileLocation, FilePath};
use crate::model::knowledge::KnowledgeTree;

pub struct LinksDecorator {
    root: Url,
    line_suffix: String,
}

impl LinksDecorator {
    pub fn new(root: &Url) -> LinksDecorator {
        let line_suffix = match root.domain() {
            Some("github.com") => "#L{}",
            Some("gitlab.com") => "#L{}",
            _ => ""
        }.to_string();

        let root = if root.path().ends_with("/") {
            root.clone()
        } else {
            root.join("/").unwrap()
        };

        LinksDecorator {
            root,
            line_suffix,
        }
    }

    fn wrap(&self, l: &mut FileLocation) -> Result<()> {
        println!("Processing link {:?}", &l);

        match l.path() {
            FilePath::Relative(p) => {
                l.replace_path(
                    FilePath::AbsoluteUrl(
                        self.root
                            .join(p.to_str().unwrap())?
                            .join(&std::fmt::format(format_args!(self.line_suffix, l.line())))?
                    )
                );
            },
            _ => {}
        }

        Ok(())
    }
}

impl Decorator for LinksDecorator {
    fn decorate(&self, tree: &mut KnowledgeTree) -> Result<()> {
        tree.visit_mut(&|node: &mut KnowledgeTree| {
            for n in node.notes_mut() {
                let mut l = n.location_mut();
                self.wrap(l)?;
            };

            for l in node.extra_mut() {
                self.wrap(l)?;
            };

            Ok(())
        })
    }
}