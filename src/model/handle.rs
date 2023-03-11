use anyhow::{Result, anyhow};

pub type HandlePart = String;

#[derive(PartialEq, Eq, Debug, Clone)]
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

    pub fn build_from_parts(parts: Vec<&str>) -> Result<Handle> {
        let parts: Vec<HandlePart> = parts.iter().map(|p| p.to_string()).collect();

        if parts.is_empty() {
            return Err(anyhow!("Empty handle".to_string()))
        }

        if parts.iter().any(|p| p.is_empty()) {
            return Err(anyhow!("Empty handle part"))
        }

        Ok(Handle { parts })
    }

    pub fn build_from_str(s: &str) -> Result<Handle> {
        let parts: Vec<_> = s.split('/').map(|p| p.trim()).collect();

        Self::build_from_parts(parts)
    }
}
