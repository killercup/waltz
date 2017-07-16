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
    #[structopt(short = "o", long = "target_dir", default_value = "examples")]
    target_dir: String,
    /// Enable logging, use multiple `v`s to increase verbosity
    #[structopt(short = "v", long = "verbose")]
    verbosity: u64,
    /// Run blocks marked as `run=cmd` with `cmd` while writing files
    #[structopt(short = "r", long = "run", default_value = "false")]
    run: bool,
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

        if let Some(cmd) = code_block.run() {
            use std::process::Command;
            let filename = code_block.filename().unwrap();

            let test = Command::new(&cmd)
                .args(&[&filename])
                .current_dir(target_directory)
                .output()?;

            if !test.status.success() {
                error!("Script {} failed.\nStdout:\n{}\n\nStderr:\n{}",
                    filename,
                    String::from_utf8_lossy(&test.stdout),
                    String::from_utf8_lossy(&test.stderr)
                );
                bail!("Failed to run `{}` script. Aborting.", filename);
            }
        }
    }

    Ok(())
});
