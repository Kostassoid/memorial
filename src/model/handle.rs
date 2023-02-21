use anyhow::{Result, anyhow};

pub type HandlePart = String;

pub struct Handle {
    parts: Vec<HandlePart>
}

impl Handle {
    pub fn parts(&self) -> &Vec<HandlePart> {
        &self.parts
    }
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
