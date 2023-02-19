use std::collections::HashMap;
use std::path::PathBuf;
use std::string::ToString;
use anyhow::{Result, anyhow};

type HandlePart = String;

struct Handle {
    parts: Vec<HandlePart>
}

impl Handle {
    pub const ROOT_PART: &'static str = "";

    pub fn build_from(s: &str) -> Result<Handle> {
        let parts: Vec<_> = s.split('/').map(|p| p.trim().to_string()).collect();

        if parts.is_empty() {
            return Err(anyhow!("Empty handle".to_string()))
        }

        if parts.iter().any(|p| p.is_empty()) {
            return Err(anyhow!("Empty handle part"))
        }

        Ok(Handle { parts })
    }
}

//todo: rename
struct Evidence {
    handle: Handle,
    file_path: PathBuf,
    file_line: u32,
    body: Option<String>,
    mentions: Vec<Handle>
}

struct KnowledgeNode {
    handle_part: HandlePart,
    evidence: Vec<Evidence>,
    children: HashMap<HandlePart, Box<KnowledgeNode>>
}

impl KnowledgeNode {
    fn empty() -> KnowledgeNode {
        KnowledgeNode {
            handle_part: Handle::ROOT_PART.to_string(),
            evidence: vec![],
            children: Default::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn knowledge_tree_build() {
        let kt = KnowledgeNode::empty();

        assert_eq!(kt.children.len(), 0)
    }
}