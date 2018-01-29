extern crate pulldown_cmark;
extern crate waltz;

#[macro_use] extern crate quicli;
use quicli::prelude::*;

use pulldown_cmark::Parser;

/// Extract code blocks from Markdown and save them as files.
#[derive(StructOpt, Debug)]
struct Cli {
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

main!(|args: Cli, log_level: verbosity| {
    info!("Read file `{}`", args.input_file);
    let input = read_file(&args.input_file)?;
    let parser = Parser::new(&input);

    let code_blocks = waltz::extract_code_blocks(parser)?;

    info!("Found {} code blocks (not all might have file names)", code_blocks.len());

    // Output files
    let target_directory = &args.target_dir;

    for code_block in code_blocks.iter().filter(|cb| cb.has_filename()) {
        code_block.to_file(target_directory)
            .with_context(|e| format!("Error writing code block to file: {}", e))?;

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
});
