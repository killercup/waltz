use std::default::Default;
use std::fs::{File, create_dir_all};
use std::io::Write;
use std::path::Path;

use errors::*;

/// Markdown code block
///
/// This is the 'final' representation, with the filename and code already
/// extracted.
#[derive(Debug, Clone)]
pub struct CodeBlock {
    filename: Option<String>,
    content: String,
}

impl Default for CodeBlock {
    fn default() -> Self {
        CodeBlock { filename: None, content: String::new() }
    }
}

impl CodeBlock {
    /// Set the code block's filename
    pub fn set_filename(&mut self, filename: String) {
        self.filename = if filename.is_empty() { None } else { Some(filename) };
    }

    /// Does the code block has a (non-empty) filename?
    pub fn has_filename(&self) -> bool {
        match self.filename {
            Some(ref f) if !f.is_empty() => true,
            _ => false,
        }
    }

    /// Get the filename, or, if it doesn't exist, a place holder.
    pub fn filename(&self) -> &str {
        match self.filename {
            Some(ref f) if !f.is_empty() => &f,
            _ => "<unnamed>",
        }
    }

    /// Get the codeblock's content
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Add to the code block's content
    pub fn push_content(&mut self, new_content: &str) {
        self.content.push_str(new_content);
    }

    /// Write codeblock to a file in directory `root`
    pub fn to_file<P: AsRef<Path>>(&self, root: P) -> Result<File> {
        if !self.has_filename() {
            bail!("Can't create file from code block with empty file name");
        }

        let path = Path::new(root.as_ref()).join(&self.filename());
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

#[cfg(test)]
mod test {
    extern crate unindent;
    use self::unindent::unindent;

    #[test]
    fn parsing() {
        let example = unindent(r#"
        # Lorem ipsum

        ## Shell

        ```bash
        $ echo "yeah!"
        ```

        ## A Rust example

        ```rust,file=src/lib.rs
        fn main() {
            println!("Dolor sit amet");
        }
        ```
        "#);

        let markdown = ::pulldown_cmark::Parser::new(&example);
        let code_blocks = ::extract_code_blocks(markdown).unwrap();

        assert_eq!(code_blocks.len(), 2);
        assert!(!code_blocks[0].has_filename());
        assert_eq!(code_blocks[0].content().trim(), r#"$ echo "yeah!""#);

        assert_eq!(code_blocks[1].filename(), "src/lib.rs".to_string());
        assert!(code_blocks[1].content().contains("Dolor sit amet"));
    }
}
