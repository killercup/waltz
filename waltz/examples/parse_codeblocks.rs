extern crate pulldown_cmark;
extern crate waltz;

use std::fs::File;
use std::io::Read;

fn main() {
    let example = {
        let mut res = String::new();
        let mut f = File::open("examples/simple_guide/GettingStarted.md").unwrap();
        f.read_to_string(&mut res).unwrap();
        res
    };

    let markdown = pulldown_cmark::Parser::new(&example);

    let code_blocks = waltz::extract_code_blocks(markdown).unwrap();

    for code_block in code_blocks {
        println!("{:?}", code_block.filename());
    }
}
