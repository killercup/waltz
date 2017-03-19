error_chain! {
    foreign_links {
        Cli(::clap::Error);
        Logging(::log::SetLoggerError);
        Waltz(::waltz::Error);
    }
}
