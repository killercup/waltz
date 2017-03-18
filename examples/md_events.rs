extern crate pulldown_cmark;

use std::fs::File;
use std::io::Read;

fn debug_markdown_events(markdown: &str) {
    let parser = pulldown_cmark::Parser::new(markdown);
    for event in parser {
        println!("{:?}", event);
    }
}

fn main() {
    let example = {
        let mut res = String::new();
        let mut f = File::open("examples/simple_guide/GettingStarted.md").unwrap();
        f.read_to_string(&mut res).unwrap();
        res
    };

    debug_markdown_events(&example);
}
