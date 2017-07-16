error_chain! {
    foreign_links {
        Cli(::structopt::clap::Error);
        Logging(::log::SetLoggerError);
        Waltz(::waltz::Error);
    }
}
