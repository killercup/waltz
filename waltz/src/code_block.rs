use std::default::Default;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::Write;
use std::path::Path;

use code_flags::CodeFlags;

/// Markdown code block
///
/// This is the 'final' representation, with the filename and code already
/// extracted.
#[derive(Debug, Clone)]
pub struct CodeBlock {
    flags: Option<CodeFlags>,
    content: String,
}

impl Default for CodeBlock {
    fn default() -> Self {
        CodeBlock {
            flags: None,
            content: String::new(),
        }
    }
}

impl CodeBlock {
    /// Set the code block's filename
    pub(crate) fn set_flags(&mut self, flags: CodeFlags) {
        self.flags = Some(flags);
    }

    /// Does the code block have a (non-empty) filename?
    pub fn has_filename(&self) -> bool {
        self.filename().is_some()
    }

    /// Get the filename if it exists
    pub fn filename(&self) -> Option<String> {
        if let Some(ref flags) = self.flags {
            flags.filename()
        } else {
            None
        }
    }

    /// Get the `run` flag if it exists
    pub fn run(&self) -> Option<String> {
        if let Some(ref flags) = self.flags {
            flags.run()
        } else {
            None
        }
    }

    /// Get the codeblock's content
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Add to the code block's content
    pub(crate) fn push_content(&mut self, new_content: &str) {
        self.content.push_str(new_content);
    }

    /// Write codeblock to a file in directory `root`
    pub fn to_file<P: AsRef<Path>>(&self, root: P) -> Result<File, ::failure::Error> {
        let filename = if let Some(f) = self.filename() {
            f
        } else {
            bail!("Can't create file from code block with empty file name")
        };

        let path = Path::new(root.as_ref()).join(filename);
        let parent = match path.parent() {
            Some(p) => p,
            None => bail!("Can't create file for code block, path has no parent directory"),
        };

        create_dir_all(parent)?;

        let mut f = OpenOptions::new().create(true).append(true).open(&path)?;
        f.write_all(self.content.as_bytes())?;

        info!("Wrote file {:?}", path);
        Ok(f)
    }
}

#[cfg(test)]
mod test {
    extern crate unindent;
    use self::unindent::unindent;

    #[test]
    fn parsing() {
        let example = unindent(
            r#"
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
        "#,
        );

        let markdown = ::pulldown_cmark::Parser::new(&example);
        let code_blocks = ::extract_code_blocks(markdown).unwrap();

        assert_eq!(code_blocks.len(), 2);
        assert!(!code_blocks[0].has_filename());
        assert_eq!(code_blocks[0].content().trim(), r#"$ echo "yeah!""#);

        assert_eq!(code_blocks[1].filename(), Some("src/lib.rs".to_string()));
        assert!(code_blocks[1].content().contains("Dolor sit amet"));
    }
}
