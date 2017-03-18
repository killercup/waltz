extern crate pulldown_cmark;

extern crate regex;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate log;

use pulldown_cmark::{Event, Tag};

mod errors;
use errors::*;

mod code_block;
use code_block::CodeBlock;

mod code_flags;

#[derive(Debug, Clone, Copy)]
enum Location {
    SomewhereUnimportant,
    InCodeBlock,
}

pub fn extract_code_blocks<'md, I: Iterator<Item=Event<'md>>>(md_events: I) -> Result<Vec<CodeBlock>> {
    let mut code_blocks = Vec::new();
    let mut location = Location::SomewhereUnimportant;
    let mut current_code_block = CodeBlock::default();

    for event in md_events {
        match (event, location) {
            (Event::Start(Tag::CodeBlock(flags)), Location::SomewhereUnimportant) => {
                location = Location::InCodeBlock;
                if let Some(filename) = code_flags::get_filename(&flags) {
                    info!("found code block with file name `{}`", filename);
                    current_code_block.filename = filename;
                }
            },
            (Event::Text(code), Location::InCodeBlock) => {
                current_code_block.content.push_str(&code);
            },
            (Event::End(Tag::CodeBlock(_lang)), Location::InCodeBlock) => {
                location = Location::SomewhereUnimportant;
                info!("end of code block for file `{}`", current_code_block.filename());
                code_blocks.push(current_code_block.clone());
                current_code_block = CodeBlock::default();
            },
            _ => {},
        }
    }

    Ok(code_blocks)
}

