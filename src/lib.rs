extern crate pulldown_cmark;
extern crate regex;

#[macro_use] extern crate log;

use std::default::Default;
use std::io::{Result as IoResult, Write};
use std::fs::{File, create_dir_all};
use std::path::Path;

use pulldown_cmark::{Event, Tag};
use regex::RegexBuilder;

#[derive(Debug, Clone)]
pub struct CodeBlock {
    file_name: String,
    content: String,
}

impl Default for CodeBlock {
    fn default() -> Self {
        CodeBlock { file_name: "".into(), content: "".into() }
    }
}

impl CodeBlock {
    pub fn to_file<P: AsRef<Path>>(&self, root: P) -> IoResult<File> {
      let path = Path::new(root.as_ref()).join(&self.file_name);
      try!(create_dir_all(path.parent().unwrap()));
      let mut f = try!(File::create(&path));
      try!(f.write_all(self.content.as_bytes()));
      Ok(f)
    }
}

#[derive(Debug, Clone, Copy)]
enum Location {
    SomewhereUnimportant,
    InFigure,
    InCodeBlock,
}

pub fn extract_code_blocks<'md, I: Iterator<Item=Event<'md>>>(md_events: I) -> Vec<CodeBlock> {
    let mut code_blocks = Vec::new();
    let mut location = Location::SomewhereUnimportant;
    let mut current_code_block = CodeBlock::default();

    let figcaption = RegexBuilder::new("<figure>(.*?)<figcaption>(?P<path>.*?)</figcaption>(.*)")
        .case_insensitive(true)
        .multi_line(true)
        .dot_matches_new_line(true)
        .build()
        .unwrap();

    for event in md_events {
        match (event, location) {
            (Event::Html(html), Location::SomewhereUnimportant) => {
                if let Some(matches) = figcaption.captures(&html) {
                    if let Some(path) = matches.name("path") {
                        location = Location::InFigure;
                        current_code_block.file_name = path.as_str().into();
                    } else {
                        warn!("No file path in figcaption: `{}`", html);
                    }
                } else {
                    warn!("No figcaption in figure: `{}`", html);
                }
            },
            (Event::Start(Tag::CodeBlock(lang)), Location::InFigure) => {
                location = Location::InCodeBlock;
                info!("found code block of lang `{}` for file name `{}`", lang, current_code_block.file_name);
            },
            (Event::Text(code), Location::InCodeBlock) => {
                current_code_block.content.push_str(&code);
            },
            (Event::End(Tag::CodeBlock(_lang)), Location::InCodeBlock) => {
                location = Location::InFigure;
                info!("end of code block for file `{}`", current_code_block.file_name);
                code_blocks.push(current_code_block.clone());
            },
            (Event::Html(html), Location::InFigure) => {
                if html.contains("</figure>") {
                    location = Location::SomewhereUnimportant;
                    current_code_block = CodeBlock::default();
                }
            },
            _ => {},
        }
    }

    code_blocks
}
