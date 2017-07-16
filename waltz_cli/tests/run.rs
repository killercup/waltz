mod utils;
use utils::*;

#[test]
fn run_script() {
    given(r#"
        # Getting started

        First of all, create a simple `Cargo.toml` file:

        ```toml,file=Cargo.toml
        [package]
        authors = ["Pascal Hertleif <killercup@gmail.com>"]
        name = "foo"
        version = "0.1.0"
        ```

        Now, let us define our first data structure by putting this into our
        `src/lib.rs`:

        ```rust,file=src/lib.rs
        struct Foo {
            x: i32,
        }
        ```

        Very good. Let's make sure it actually works:

        ```rust,file=src/lib.rs
        #[test]
        fn foo_test() {
            let foo = Foo { x: 42 };
            assert_eq!(foo.x, 42);
        }
        ```

        It's easy to run these tests with cargo:

        ```sh,file=scripts/0-test.sh,run=sh
        cargo test
        ```
    "#)
    .running(waltz_test)
    .creates(file("Cargo.toml"))
    .creates(file("src/lib.rs"))
    .creates(file("scripts/0-test.sh"))
    ;
}

#[test]
fn failing_script() {
    given("
        You can't run `cargo` in a directory with not `Cargo.toml`:

        ```sh,file=foo.sh,run=sh
        cargo build
        ```
    ")
   .running(|cwd|
        CliAssert::main_binary()
        .with_args(&[
            "-vvv", "-r",
            "-o", cwd.to_str().unwrap(),
            cwd.join("test.md").to_str().unwrap(),
        ])
        .fails()
        .prints_error("Script foo.sh failed.")
        .prints_error("could not find `Cargo.toml` in")
   )
   ;
}

#[test]
fn script_with_side_effects() {
    given("
        You can easily create empty file on the command line using

        ```sh,file=scripts/0-touch.sh,run=sh
        touch foobar
        ```
    ")
    .running(waltz_test)
    .creates(file("foobar"))
    ;
}
