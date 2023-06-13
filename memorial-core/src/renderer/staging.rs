use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::Result;

/*@[Core/Renderer/Staging]:
`StagingArea` acts as an intermediate temp file system for keeping the rendered files until the rendering
is complete. The files can then be written down to the final location in one go.

Writing down the rendered files directly can be problematic. In case of errors these files
can end up being arbitrarily broken. Having the documentation checked in into VCS can help with
restoring the original state, but this is an unnecessary limitation. Also it's still an extra step
which can be avoided.
*/
pub struct StagingArea {
    staged: HashMap<PathBuf, StagedFile>,
}

impl StagingArea {
    pub fn new() -> StagingArea {
        StagingArea {
            staged: Default::default(),
        }
    }

    pub fn open_as_new<P: AsRef<Path>>(&mut self, path: P) -> &mut StagedFile {
        let p: PathBuf = path.as_ref().to_path_buf();
        self.staged.insert(p.clone(), StagedFile::new());
        self.staged.get_mut(&p).unwrap()
    }

    pub fn flush_to_os_fs<P: AsRef<Path>>(&mut self, root: P) -> Result<()> {
        for (path, file) in self.staged.drain() {
            let full_path = if path.is_relative() {
                root.as_ref().join(path)
            } else {
                path
            };

            fs::write(full_path, file.contents)?;
        }

        Ok(())
    }

    pub fn flush_to_stdout(&mut self) -> Result<()> {
        for (path, file) in self.staged.drain() {
            println!(
                "Staged file: {}\n{}\n\n",
                &path.display(),
                String::from_utf8(file.contents)?
            )
        }

        Ok(())
    }

    pub fn open<P: AsRef<Path>>(&self, path: P) -> Option<&StagedFile> {
        self.staged.get(&path.as_ref().to_path_buf())
    }
}

/*@[Core/Renderer/Staging]:
Staging in the current implementation stores data in memory. This approach simplifies design but
obviously won't scale well. For the future versions, some memory independent storage should be used.
E.g. `/tmp`.
*/
pub struct StagedFile {
    contents: Vec<u8>,
}

impl StagedFile {
    pub fn new() -> StagedFile {
        StagedFile {
            contents: Default::default(),
        }
    }

    pub fn contents(&self) -> &Vec<u8> {
        &self.contents
    }
}

impl Write for StagedFile {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.contents.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        //noop
        Ok(())
    }
}

impl std::fmt::Write for StagedFile {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        match self.write(s.as_bytes()) {
            Err(_) => Err(std::fmt::Error),
            _ => Ok(()),
        }
    }
}
