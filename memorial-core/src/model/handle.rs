use anyhow::{anyhow, Result};
use std::fmt::{Display, Formatter};

pub type HandlePart = String;

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub struct Handle {
    parts: Vec<HandlePart>,
}

impl Handle {
    pub const ROOT: Handle = Handle { parts: vec![] };

    pub fn parts(&self) -> &Vec<HandlePart> {
        &self.parts
    }
}

impl Handle {
    pub fn from_parts(parts: Vec<HandlePart>) -> Result<Handle> {
        if parts.is_empty() {
            return Err(anyhow!("Empty handle".to_string()));
        }

        if parts.iter().any(|p| p.is_empty()) {
            return Err(anyhow!("Empty handle part"));
        }

        Ok(Handle { parts })
    }

    pub fn from_str(s: &str) -> Result<Handle> {
        let parts: Vec<_> = s.split('/').map(|p| p.trim()).collect();

        Self::from_parts(parts.into_iter().map(|s| s.to_owned()).collect())
    }

    //todo: incomplete
    pub fn as_url_safe_string(&self) -> String {
        self.parts
            .join("+")
            .replace(" ", "-")
            .replace("\t", "-")
            .replace("\n", "-")
            .to_lowercase()
    }

    pub fn join(&self, part: HandlePart) -> Result<Handle> {
        let mut new_parts = Vec::with_capacity(self.parts.len() + 1);
        new_parts.clone_from(&self.parts);
        new_parts.push(part);
        Handle::from_parts(new_parts)
    }
}

impl Display for Handle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.parts.join(" / "))
    }
}
