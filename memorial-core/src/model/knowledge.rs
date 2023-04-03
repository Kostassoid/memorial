use super::handle::*;
use super::note::Note;
use crate::model::file_location::FileLocation;
use crate::model::note::NoteSpan;
use anyhow::Result;
use std::collections::{BTreeMap, HashMap, HashSet};

#[derive(Debug)]
pub struct KnowledgeTree {
    handle: Handle,
    attributes: HashMap<String, String>,
    mentions: HashSet<Handle>,
    notes: Vec<Note>,
    extra: Vec<FileLocation>,
    children: BTreeMap<HandlePart, Box<KnowledgeTree>>,
}

impl KnowledgeTree {
    pub fn at(handle: Handle) -> KnowledgeTree {
        KnowledgeTree {
            handle,
            attributes: Default::default(),
            mentions: Default::default(),
            notes: vec![],
            extra: vec![],
            children: Default::default(),
        }
    }

    pub fn root() -> KnowledgeTree {
        Self::at(Handle::ROOT)
    }

    pub fn is_empty(&self) -> bool {
        self.notes.is_empty() && self.children.is_empty()
    }

    pub fn handle(&self) -> &Handle {
        &self.handle
    }
    pub fn children(&self) -> &BTreeMap<HandlePart, Box<KnowledgeTree>> {
        &self.children
    }

    pub fn attributes(&self) -> &HashMap<String, String> {
        &self.attributes
    }

    pub fn mentions(&self) -> &HashSet<Handle> {
        &self.mentions
    }

    pub fn notes(&self) -> &Vec<Note> {
        &self.notes
    }

    pub fn notes_mut(&mut self) -> &mut Vec<Note> {
        &mut self.notes
    }

    pub fn extra(&self) -> &Vec<FileLocation> {
        &self.extra
    }

    pub fn extra_mut(&mut self) -> &mut Vec<FileLocation> {
        &mut self.extra
    }

    pub fn find_node_mut(&mut self, handle: &Handle) -> &mut KnowledgeTree {
        let mut node = self;

        let mut temp_handle = Handle::ROOT;

        for p in handle.parts() {
            temp_handle = temp_handle.join(p.clone()).unwrap(); //todo: this
            let children = &mut node.children;
            if children.contains_key(p) {
                node = children.get_mut(p).unwrap();
            } else {
                let new_node = KnowledgeTree::at(temp_handle.clone());

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

    pub fn add_note(&mut self, handle: &Handle, note: Note) {
        for s in note.spans() {
            match s {
                NoteSpan::Link(h) => self.register_mention(&h, handle),
                _ => {}
            }
        }

        let node = self.find_node_mut(handle);

        if note.spans().is_empty() {
            node.extra.push(note.location().clone());
        } else {
            node.notes.push(note);
        }
    }

    pub fn merge_attributes(&mut self, handle: &Handle, attributes: HashMap<String, String>) {
        let node = self.find_node_mut(handle);

        for (k, v) in attributes {
            node.attributes.insert(k, v);
        }
    }

    fn register_mention(&mut self, to: &Handle, from: &Handle) {
        let node = self.find_node_mut(to);

        node.mentions.insert(from.clone());
    }

    #[allow(dead_code)]
    pub fn visit<F>(&self, f: &F) -> Result<()>
    where
        F: Fn(&KnowledgeTree) -> Result<()>,
    {
        f(self)?;

        for (_, n) in &self.children {
            n.visit(f)?;
        }

        Ok(())
    }

    pub fn visit_mut<F>(&mut self, f: &F) -> Result<()>
    where
        F: Fn(&mut KnowledgeTree) -> Result<()>,
    {
        f(self)?;

        for (_, n) in &mut self.children {
            n.visit_mut(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::model::note::NoteSpan;
    use std::cell::RefCell;

    #[test]
    fn knowledge_tree_empty() {
        let kt = KnowledgeTree::root();

        assert_eq!(kt.children.len(), 0);
        assert!(kt.is_empty())
    }

    #[test]
    fn adding_non_empty_notes() {
        let mut kt = KnowledgeTree::root();

        let handle1 = Handle::from_str("a/b/c").unwrap();
        let handle2 = Handle::from_str("a/b/d").unwrap();

        let note1a = Note::new(
            FileLocation::new_relative("file1.go", 333),
            vec![NoteSpan::Text("Note 1".to_string())],
        );

        let note1b = Note::new(
            FileLocation::new_relative("file1.go", 444),
            vec![NoteSpan::Text("Note 1b".to_string())],
        );

        let note2 = Note::new(
            FileLocation::new_relative("file2.go", 333),
            vec![NoteSpan::Text("Note 2".to_string())],
        );

        kt.add_note(&handle1, note1a.clone());
        kt.add_note(&handle1, note1b.clone());
        kt.add_note(&handle2, note2.clone());

        assert!(kt
            .children
            .get("a")
            .unwrap()
            .children
            .get("b")
            .unwrap()
            .children
            .get("c")
            .is_some());
        assert!(kt
            .children
            .get("a")
            .unwrap()
            .children
            .get("b")
            .unwrap()
            .children
            .get("d")
            .is_some());

        assert_eq!(vec!(note1a, note1b), kt.find_node(&handle1).unwrap().notes);
        assert_eq!(vec!(note2), kt.find_node(&handle2).unwrap().notes);
    }

    #[test]
    fn adding_empty_notes() {
        let mut kt = KnowledgeTree::root();

        let handle = Handle::from_str("a/b/c").unwrap();

        let note1 = Note::new(FileLocation::new_relative("file1.go", 333), vec![]);

        let note2 = Note::new(FileLocation::new_relative("file1.go", 444), vec![]);

        kt.add_note(&handle, note1.clone());
        kt.add_note(&handle, note2.clone());

        let node = kt.find_node(&handle).unwrap();

        assert!(node.notes.is_empty());
        assert_eq!(
            vec!(
                FileLocation::new_relative("file1.go", 333),
                FileLocation::new_relative("file1.go", 444)
            ),
            node.extra,
        )
    }

    #[test]
    fn merging_attributes() {
        let mut kt = KnowledgeTree::root();

        let handle = Handle::from_str("a/b/c").unwrap();

        let attributes1 = HashMap::from([
            ("k1".to_string(), "v1".to_string()),
            ("k2".to_string(), "v2".to_string()),
        ]);
        let attributes2 = HashMap::from([
            ("k3".to_string(), "v3".to_string()),
            ("k2".to_string(), "v2".to_string()),
        ]);

        kt.merge_attributes(&handle, attributes1.clone());
        kt.merge_attributes(&handle, attributes2.clone());

        let node = kt.find_node(&handle).unwrap();

        assert!(node.notes.is_empty());
        assert_eq!(
            HashMap::from([
                ("k1".to_string(), "v1".to_string()),
                ("k2".to_string(), "v2".to_string()),
                ("k3".to_string(), "v3".to_string()),
            ]),
            node.attributes,
        )
    }

    #[test]
    fn walking_a_tree() {
        let mut kt = KnowledgeTree::root();

        kt.find_node_mut(&Handle::from_str("a/b/c").unwrap());
        kt.find_node_mut(&Handle::from_str("x/y/z").unwrap());
        kt.find_node_mut(&Handle::from_str("a/b/d").unwrap());

        let visited: RefCell<Vec<String>> = RefCell::new(vec![]);
        kt.visit_mut(&|n| {
            visited.borrow_mut().push(n.handle.to_string());
            Ok(())
        })
        .unwrap();

        assert_eq!(
            vec!(
                "",
                "a",
                "a / b",
                "a / b / c",
                "a / b / d",
                "x",
                "x / y",
                "x / y / z"
            ),
            visited.into_inner()
        );
    }
}
