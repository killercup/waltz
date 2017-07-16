extern crate pulldown_cmark;
extern crate waltz;

extern crate structopt;
#[macro_use] extern crate structopt_derive;

#[macro_use] extern crate error_chain;

#[macro_use] extern crate log;
extern crate loggerv;

use std::fs::File;
use std::io::Read;

use structopt::StructOpt;

use pulldown_cmark::Parser;

mod errors;
use errors::*;

/// Extract code blocks from Markdown and save them as files.
#[derive(StructOpt, Debug)]
#[structopt(name = "waltz", author = "Pascal Hertlei")]
struct App {
    /// The target directory
    #[structopt(short = "o", default_value = "examples")]
    #[structopt(short = "o", long = "target_dir", default_value = "examples")]
    target_dir: String,
    /// Enable logging, use multiple `v`s to increase verbosity
    #[structopt(short = "v")]
    #[structopt(short = "v", long = "verbose")]
    verbosity: u64,
    /// The input markdown file
    #[structopt(name = "FILES")]
    input_file: String,
}

quick_main!(|| -> Result<()> {
    let args = App::from_args();

    loggerv::init_with_verbosity(args.verbosity)?;

    // Parse markdown file
    let input = {
        let input_file = &args.input_file;

        let mut res = String::new();
        let mut f = File::open(input_file)
            .chain_err(|| format!("Error opening file `{}`", input_file))?;
        f.read_to_string(&mut res)
            .chain_err(|| format!("Error reading file `{}`", input_file))?;
        info!("Read file `{}`", input_file);
        res
    };
    let parser = Parser::new(&input);

    let code_blocks = waltz::extract_code_blocks(parser)?;

    info!("Found {} code blocks (not all might have file names)", code_blocks.len());

    // Output files
    let target_directory = &args.target_dir;

    for code_block in code_blocks.iter().filter(|cb| cb.has_filename()) {
        code_block.to_file(target_directory)
            .chain_err(|| "Error writing code block to file")?;
    }

    Ok(())
});
