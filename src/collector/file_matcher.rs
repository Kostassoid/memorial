use std::path::Path;

#[derive(Eq, Hash, PartialEq)]
pub enum FileTypeMatcher {
    Extension(String),
}

impl FileTypeMatcher {
    pub fn is_match<P: AsRef<Path>>(&self, path: P) -> bool {
        match self {
            FileTypeMatcher::Extension(ext) =>
                path.as_ref().extension()
                    // .map(|e| e.eq_ignore_ascii_case(ext))
                    .map(|e| e.eq_ignore_ascii_case(ext))
                    .unwrap_or(false)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn match_by_extension_ascii() {
        let matcher = FileTypeMatcher::Extension("ext".to_string());

        assert!(matcher.is_match("file.ext"));
        assert!(matcher.is_match("file.Ext"));
        assert!(matcher.is_match("path/to/file.ext"));

        assert!(!matcher.is_match(".ext")); // not an extension
        assert!(!matcher.is_match("file.extt"));
        assert!(!matcher.is_match("file.ex"));
    }

    #[test]
    fn match_by_extension_unicode() {
        let matcher = FileTypeMatcher::Extension("йцщ".to_string());

        assert!(matcher.is_match("file.йцщ"));
        assert!(matcher.is_match("path/to/file.йцщ"));

        assert!(!matcher.is_match("file.ЙЦЩ")); // not supported currently
    }
}