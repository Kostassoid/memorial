use std::collections::HashMap;
use std::fmt::Debug;
use std::path::PathBuf;
use std::string::ToString;
use url::Url;
use anyhow::Result;
use strfmt::{DisplayStr, Format};
use crate::decorators::Decorator;
use crate::model::file_location::{FileLocation, FilePath};
use crate::model::knowledge::KnowledgeTree;

pub struct LinksDecorator {
    root: String,
    format: String,
}

const DEFAULT_FORMAT: &str = "{root}/{path}";

impl LinksDecorator {
    pub fn new(root: String, format: Option<String>) -> Result<LinksDecorator> {
        let format = format
            .or_else(|| { Self::resolve_format(&root) })
            .unwrap_or(DEFAULT_FORMAT.to_string());

        let normalized_root = root.trim_end_matches('/').to_string();

        Ok(LinksDecorator {
            root: root.trim_end_matches('/').to_string(),
            format,
        })
    }

    fn resolve_format(root: &str) -> Option<String> {
        if root.contains("github.com") || root.contains("gitlab.com") {
            return Some("{root}/blob/master/{path}#L{line}".to_string())
        }

        None
    }

    fn wrap(&self, l: &mut FileLocation) -> Result<()> {
        match l.path() {
            FilePath::Relative(p) => {
                let line_str = l.line().to_string();
                let vars: HashMap<String, &str> = HashMap::from([
                    ("root".to_string(), self.root.as_str()),
                    ("path".to_string(), p.to_str().unwrap()),
                    ("line".to_string(), &line_str),
                ]);

                let url = Url::parse(&self.format.format(&vars)?)?;

                l.replace_path(FilePath::AbsoluteUrl(url));
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