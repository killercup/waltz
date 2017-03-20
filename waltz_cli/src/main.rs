extern crate pulldown_cmark;
extern crate waltz;

#[macro_use] extern crate clap;
#[macro_use] extern crate error_chain;

#[macro_use] extern crate log;
extern crate loggerv;

use std::fs::File;
use std::io::Read;

use pulldown_cmark::Parser;

mod errors;
use errors::*;

mod cli;

fn main() {
    if let Err(error) = try_main() {
        error!("{:?}", error);
        std::process::exit(1);
    }
}

fn try_main() -> Result<()> {
    let args = cli::app().get_matches();

    loggerv::init_with_verbosity(args.occurrences_of("v"))?;

    // CLI args
    let input_file = args.value_of("input_file").unwrap();
    let target_directory = args.value_of("target_dir")
        .unwrap_or("examples");

    // Parse markdown file
    let input = {
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
    for code_block in code_blocks.iter().filter(|cb| cb.has_filename()) {
        code_block.to_file(target_directory)
            .chain_err(|| "Error writing code block to file")?;
    }

    Ok(())
}
