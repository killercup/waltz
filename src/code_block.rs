use std::default::Default;
use std::fs::{File, create_dir_all};
use std::io::Write;
use std::path::Path;

use errors::*;

#[derive(Debug, Clone)]
pub struct CodeBlock {
    pub filename: String,
    pub content: String,
}

impl Default for CodeBlock {
    fn default() -> Self {
        CodeBlock { filename: "".into(), content: "".into() }
    }
}

impl CodeBlock {
    pub fn filename(&self) -> &str {
        if self.filename.is_empty() {
            "<unnamed>"
        } else {
            &self.filename
        }
    }

    pub fn to_file<P: AsRef<Path>>(&self, root: P) -> Result<File> {
        if self.filename.is_empty() {
            bail!("Can't create file from code block with empty file name");
        }

        let path = Path::new(root.as_ref()).join(&self.filename);
        let parent = match path.parent() {
            Some(p) => p,
            None => bail!("Can't create file for code block, path has no parent directory"),
        };

        create_dir_all(parent)?;
        let mut f = File::create(&path)?;
        f.write_all(self.content.as_bytes())?;
        Ok(f)
    }
}
