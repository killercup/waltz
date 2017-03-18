extern crate waltz;
extern crate pulldown_cmark;

extern crate slog_stdlog;
extern crate slog_envlogger;
extern crate slog_term;
#[macro_use(o)] extern crate slog;

use std::fs::File;
use std::io::Read;

use slog::DrainExt;

fn main() {
    let term = slog_term::streamer().build();
    let drain = slog_envlogger::new(term);
    let root_logger = slog::Logger::root(drain.fuse(), o!());
    slog_stdlog::set_logger(root_logger.clone()).unwrap();

    let example = {
        let mut res = String::new();
        let mut f = File::open("examples/simple_guide/GettingStarted.md").unwrap();
        f.read_to_string(&mut res).unwrap();
        res
    };

    let markdown = pulldown_cmark::Parser::new(&example);

    let code_blocks = waltz::extract_code_blocks(markdown).unwrap();

    for code_block in code_blocks {
        println!("{:?}", code_block.filename);
    }
}
