use std::collections::{BTreeMap, HashMap};
use std::string::ToString;
use super::handle::*;
use super::note::Note;

#[derive(Debug)]
pub struct KnowledgeTree {
    attributes: HashMap<String, String>,
    notes: Vec<Note>,
    children: BTreeMap<HandlePart, Box<KnowledgeTree>>
}

impl KnowledgeTree {
    pub fn empty() -> KnowledgeTree {
        KnowledgeTree {
            attributes: Default::default(),
            notes: vec![],
            children: Default::default(),
        }
    }

    pub fn children(&self) -> &BTreeMap<HandlePart, Box<KnowledgeTree>> {
        &self.children
    }

    pub fn attributes(&self) -> &HashMap<String, String> {
        &self.attributes
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

    pub fn add(&mut self, handle: Handle, note: Note, attributes: HashMap<String, String>) -> () {
        let node = self.find_node_mut(&handle);

        node.notes.push(note);

        for (k, v) in attributes {
            node.attributes.insert(k, v);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::model::note::{FileLocation, NoteSpan};
    use super::*;

    #[test]
    fn knowledge_tree_empty() {
        let kt = KnowledgeTree::empty();

        assert_eq!(kt.children.len(), 0)
    }

    #[test]
    fn knowledge_tree_adding_records() {
        let mut kt = KnowledgeTree::empty();

        let handle = Handle::build_from_str("a/b/c").unwrap();

        let attributes = HashMap::from([("k1".to_string(), "v1".to_string())]);

        let note1 = Note::new(
            FileLocation::new("test.go", 333),
            vec!(NoteSpan::Text("Facts".to_string())),
            vec![],
        );

        let note2 = Note::new(
            FileLocation::new("test2.go", 333),
            vec!(NoteSpan::Text("Facts 2".to_string())),
            vec![],
        );

        kt.add(handle.clone(),note1.clone(), attributes.clone());
        kt.add(handle.clone(),note2.clone(), Default::default());

        assert_eq!(kt.children.len(), 1);
        assert!(kt.children.get("a").unwrap().children.get("b").unwrap().children.get("c").is_some());
        assert_eq!(vec!(note1, note2), kt.find_node(&handle).unwrap().notes);
        assert_eq!(attributes, kt.find_node(&handle).unwrap().attributes)
    }
}