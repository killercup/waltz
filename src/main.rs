extern crate slog_stdlog;
extern crate slog_envlogger;
extern crate slog_term;
#[macro_use(o)] extern crate slog;
#[macro_use] extern crate log;

extern crate pulldown_cmark;

extern crate waltz;

use std::fs::File;
use std::io::Read;

use pulldown_cmark::Parser;
use slog::DrainExt;

fn main() {
    let term = slog_term::streamer().build();
    let drain = slog_envlogger::new(term);
    let root_logger = slog::Logger::root(drain.fuse(), o!());
    slog_stdlog::set_logger(root_logger.clone()).unwrap();

    let mut args = std::env::args().skip(1);
    let input_file = args.next().expect("No input file given.\n\tUsage: waltz <input file> [<target directory>]");
    let target_directory = args.next().unwrap_or("examples".into());

    let example = {
        let mut res = String::new();
        let mut f = File::open(&input_file).expect(&format!("Error opening file `{}`", input_file));
        f.read_to_string(&mut res).expect(&format!("Error reading file `{}`", input_file));
        res
    };
    let parser = Parser::new(&example);

    let code_blocks = waltz::extract_code_blocks(parser).unwrap();

    for code_block in code_blocks.iter().filter(|x| !x.filename.is_empty()) {
        code_block.to_file(&target_directory).expect("Error writing code block to file");
    }
}
