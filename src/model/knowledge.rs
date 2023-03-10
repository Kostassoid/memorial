use std::collections::BTreeMap;
use std::string::ToString;
use anyhow::{Result, anyhow};
use super::handle::*;
use super::note::Note;

#[derive(Debug)]
pub struct KnowledgeTree {
    handle_part: HandlePart,
    notes: Vec<Note>,
    children: BTreeMap<HandlePart, Box<KnowledgeTree>>
}

impl KnowledgeTree {
    pub fn empty() -> KnowledgeTree {
        KnowledgeTree {
            handle_part: Handle::ROOT_PART.to_string(),
            notes: vec![],
            children: Default::default(),
        }
    }

    pub fn find_node_mut(&mut self, handle: &Handle) -> &mut KnowledgeTree {
        let mut node = self;

        for p in handle.parts() {
            let children = &mut node.children;
            if children.contains_key(p) {
                node = children.get_mut(p).unwrap();
            } else {
                let new_node = KnowledgeTree::empty();

                children.insert(p.clone(), Box::new(new_node));

                node = children.get_mut(p).unwrap();
            }
        }

        node
    }

    pub fn find_node(&self, handle: &Handle) -> Option<&KnowledgeTree> {
        let mut node = self;

        for p in handle.parts() {
            let children = &node.children;
            if !children.contains_key(p) {
                return None;
            }
            node = children.get(p).unwrap();
        }

        Some(node)
    }

    pub fn add(&mut self, note: Note) -> () {
        self.find_node_mut(note.handle()).notes.push(note);
    }

}

#[cfg(test)]
mod test {
    use crate::model::note::FileLocation;
    use super::*;

    #[test]
    fn knowledge_tree_empty() {
        let kt = KnowledgeTree::empty();

        assert_eq!(kt.children.len(), 0)
    }

    #[test]
    fn knowledge_tree_adding_records() {
        let mut kt = KnowledgeTree::empty();

        let note1 = Note::new(
            Handle::build_from("a/b/c").unwrap(),
            FileLocation::new("test.go", 333),
            Some("Facts".to_string()),
            vec![],
        );

        let note2 = Note::new(
            Handle::build_from("a/b/c").unwrap(),
            FileLocation::new("test2.go", 333),
            Some("Facts 2".to_string()),
            vec![],
        );

        kt.add(note1.clone());
        kt.add(note2.clone());

        assert_eq!(kt.children.len(), 1);
        assert!(kt.children.get("a").unwrap().children.get("b").unwrap().children.get("c").is_some());
        assert_eq!(kt.find_node(note1.handle()).unwrap().notes, vec!(note1, note2));
    }
}