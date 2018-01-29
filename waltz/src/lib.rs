//! Extract code files from Markdown files.
//!
//! Write guides in Markdown with code blocks that belong in several files, and
//! let _waltz_ extract the code for you so you can build/run/test it easily.
//!
//! Meant as a companion to [tango].
//!
//! [tango]: https://github.com/pnkfelix/tango

#![deny(missing_docs)]

#[macro_use] extern crate error_chain;
#[macro_use] extern crate log;
#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate pulldown_cmark;

use pulldown_cmark::{Event, Tag};

mod errors;
use errors::*;

pub use errors::Error;

mod code_block;
pub use code_block::CodeBlock;

mod code_flags;
use code_flags::CodeFlags;

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
/// let example =
///     r#"```rust,file=examples/demo.rs
///     pub const: &'static str = "Yeah!";
///     ```"#;
/// let markdown = pulldown_cmark::Parser::new(example);
/// let code_blocks = waltz::extract_code_blocks(markdown).unwrap();
/// assert_eq!(code_blocks[0].filename(), Some("examples/demo.rs".to_string()));
/// ```
pub fn extract_code_blocks<'md, I: Iterator<Item=Event<'md>>>(md_events: I) -> Result<Vec<CodeBlock>> {
    let mut code_blocks = Vec::new();
    let mut location = Location::SomewhereUnimportant;
    let mut current_code_block = CodeBlock::default();

    for event in md_events {
        match (event, location) {
            (Event::Start(Tag::CodeBlock(flags)), Location::SomewhereUnimportant) => {
                location = Location::InCodeBlock;
                let flags = flags.parse::<CodeFlags>()?;

                trace!("found code block{}",
                    if let Some(f) = flags.filename() {
                       format!(" with file name `{}`", f)
                    } else {
                        format!(" without a file name")
                    }
                );

                current_code_block.set_flags(flags);
            },
            (Event::Text(code), Location::InCodeBlock) => {
                current_code_block.push_content(&code);
            },
            (Event::End(Tag::CodeBlock(_lang)), Location::InCodeBlock) => {
                location = Location::SomewhereUnimportant;
                trace!("end of code block for file `{}`",
                    current_code_block.filename().unwrap_or_else(|| "<unnamed>".to_string())
                );
                code_blocks.push(current_code_block.clone());
                current_code_block = CodeBlock::default();
            },
            _ => {},
        }
    }

    Ok(code_blocks)
}
