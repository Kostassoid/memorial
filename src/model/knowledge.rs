use std::collections::BTreeMap;
use std::string::ToString;
use anyhow::{Result, anyhow};
use super::handle::*;
use super::note::Note;

pub struct KnowledgeNode {
    handle_part: HandlePart,
    notes: Vec<Note>,
    children: BTreeMap<HandlePart, Box<KnowledgeNode>>
}

impl KnowledgeNode {
    pub fn empty() -> KnowledgeNode {
        KnowledgeNode {
            handle_part: Handle::ROOT_PART.to_string(),
            notes: vec![],
            children: Default::default(),
        }
    }

    pub fn add(&mut self, note: Note) -> () {
        let mut node = self;
        note.handle().parts().iter_mut().for_each(|p| {
            match node.children.get_mut(p) {
                Some(n) => node = n,
                None => {
                    let mut new_node = KnowledgeNode::empty();

                    node.children.insert(p.clone(), Box::new(new_node));

                    node = &mut new_node;
                }
            }
        });

        node.notes.push(note);
    }

}

#[cfg(test)]
mod test {
    use crate::model::note::FileLocation;
    use super::*;

    #[test]
    fn knowledge_tree_empty() {
        let kt = KnowledgeNode::empty();

        assert_eq!(kt.children.len(), 0)
    }

    #[test]
    fn knowledge_tree_adding_records() {
        let mut kt = KnowledgeNode::empty();

        let note = Note::new(
            Handle::build_from("a/b/c").unwrap(),
            FileLocation::new("test.go", 333),
            Some("Facts".to_string()),
            vec![],
        );

        kt.add(note);

        assert_eq!(kt.children.len(), 1)
    }
}