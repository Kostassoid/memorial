use std::path;
use std::path::Path;
use anyhow::{Result, anyhow, Context};

pub struct PathFilter {
    include: Vec<glob::Pattern>,
    exclude: Vec<glob::Pattern>,
}

impl PathFilter {
    pub fn from_glob(include: &Vec<String>, exclude: &Vec<String>) -> Result<PathFilter> {
        let include: std::result::Result<Vec<_>, _> = include
            .into_iter()
            .map(|s| glob::Pattern::new(&s))
            .collect();
        let exclude: std::result::Result<Vec<_>, _> = exclude
            .into_iter()
            .map(|s| glob::Pattern::new(&s))
            .collect();

        Ok(PathFilter {
            include: include?,
            exclude: exclude?,
        })
    }

    pub fn is_allowed<P: AsRef<Path>>(&self, path: P) -> bool {
        let included = self.include.iter().any(|p| p.matches_path(path.as_ref()));
        let excluded = self.exclude.iter().any(|p| p.matches_path(path.as_ref()));
        included && !excluded
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn allowed_patterns() {
        // taken from https://www.atlassian.com/git/tutorials/saving-changes/gitignore
        let f = PathFilter::from_glob(
            &vec!(r"**/logs/*".to_string()), //"**/logs" would be allowed in .gitignore but not in Unix globs
            &vec![],
        ).unwrap();
        assert!(f.is_allowed(r"logs/debug.log"));
        assert!(f.is_allowed(r"logs/monday/foo.bar"));
        assert!(f.is_allowed(r"build/logs/debug.log"));
        assert!(!f.is_allowed(r"log/debug.log"));

        let f = PathFilter::from_glob(
            &vec!(r"**/logs/debug.log".to_string()),
            &vec![],
        ).unwrap();
        assert!(f.is_allowed(r"logs/debug.log"));
        assert!(f.is_allowed(r"build/logs/debug.log"));
        assert!(!f.is_allowed(r"logs/build/debug.log"));
    }
}