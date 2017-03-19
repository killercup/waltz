//! Extract code files from Markdown files.
//!
//! Write guides in Markdown with code blocks that belong in several files, and
//! let _waltz_ extract the code for you so you can build/run/test it easily.
//!
//! Meant as a companion to [tango].
//!
//! [tango]: https://github.com/pnkfelix/tango

#![deny(warnings, missing_docs)]

extern crate pulldown_cmark;
extern crate regex;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate log;

use pulldown_cmark::{Event, Tag};

mod errors;
use errors::*;

mod code_block;
pub use code_block::CodeBlock;

mod code_flags;

#[derive(Debug, Clone, Copy)]
enum Location {
    SomewhereUnimportant,
    InCodeBlock,
}


/// Extract code blocks from Markdown events
///
/// The input needs to be an Iterator as returned by [pulldown-cmark]'s parser.
///
/// [pulldown-cmark]: https://github.com/google/pulldown-cmark
///
/// # Examples
///
/// ```rust
/// extern crate waltz;
/// extern crate pulldown_cmark;
///
/// let example = r#"
///  ```rust,file=examples/demo.rs
///  pub const: &'static str = "Yeah!";
///  ```
/// "#;
/// let markdown = pulldown_cmark::Parser::new(example);
/// let code_blocks = waltz::extract_code_blocks(markdown).unwrap();
/// assert_eq!(code_blocks[0].filename(), "examples/demo.rs".to_string());
/// ```
pub fn extract_code_blocks<'md, I: Iterator<Item=Event<'md>>>(md_events: I) -> Result<Vec<CodeBlock>> {
    let mut code_blocks = Vec::new();
    let mut location = Location::SomewhereUnimportant;
    let mut current_code_block = CodeBlock::default();

    for event in md_events {
        match (event, location) {
            (Event::Start(Tag::CodeBlock(flags)), Location::SomewhereUnimportant) => {
                location = Location::InCodeBlock;
                if let Some(filename) = code_flags::get_filename(&flags) {
                    trace!("found code block with file name `{}`", filename);
                    current_code_block.set_filename(filename);
                } else {
                    trace!("found code block without file name");
                }
            },
            (Event::Text(code), Location::InCodeBlock) => {
                current_code_block.push_content(&code);
            },
            (Event::End(Tag::CodeBlock(_lang)), Location::InCodeBlock) => {
                location = Location::SomewhereUnimportant;
                trace!("end of code block for file `{}`", current_code_block.filename());
                code_blocks.push(current_code_block.clone());
                current_code_block = CodeBlock::default();
            },
            _ => {},
        }
    }

    Ok(code_blocks)
}
