use clap::{Arg, App};

pub fn app() -> App<'static, 'static> {
    App::new("waltz")
        .version(crate_version!())
        .author("Pascal Hertleif <killercup@gmail.com>")
        .about("Extract code blocks from Markdown and save them as files.")
        .arg(
            Arg::with_name("input_file")
                .help("The input markdown file")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("target_dir")
                .help("The target directory")
                .short("o")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .help("Enable logging, use multiple `v`s to increase verbosity")
                .multiple(true),
        )
}
