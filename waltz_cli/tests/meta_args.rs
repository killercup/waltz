extern crate assert_cli;
use assert_cli::Assert;

#[test]
fn no_args() {
    Assert::main_binary()
        .fails_with(1)
        .prints_error("The following required arguments were not provided")
        .unwrap();
}

#[test]
fn help() {
    Assert::main_binary()
        .with_args(&["--help"])
        .succeeds()
        .unwrap();

    Assert::main_binary()
        .with_args(&["-h"])
        .succeeds()
        .unwrap();
}

#[test]
fn version() {
    Assert::main_binary()
        .with_args(&["--version"])
        .succeeds()
        .unwrap();

    Assert::main_binary()
        .with_args(&["-V"])
        .succeeds()
        .unwrap();
}
