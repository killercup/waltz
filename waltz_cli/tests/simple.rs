#![allow(dead_code)]
include!("utils/lib.rs");

#[test]
fn simple() {
    given(r#"
        # Getting started

        First of all, create a simple `Cargo.toml` file:

        ```toml,file=Cargo.toml
        [package]
        authors = ["Pascal Hertleif <killercup@gmail.com>"]
        name = "foo"
        version = "0.1.0"
        ```

        Nice. Now you can put this into `src/main.rs`:

        ```rust,no_run,file=src/main.rs
        fn main() {
            println!("Hello, world!");
        }
        ```
    "#)
    .waltz()
    .creates(file("Cargo.toml").containing(r#"
        [package]
        authors = ["Pascal Hertleif <killercup@gmail.com>"]
        name = "foo"
        version = "0.1.0"
    "#))
    .creates(file("src/main.rs").containing(r#"
        fn main() {
            println!("Hello, world!");
        }
    "#))
    .cargo_run(|cmd| cmd.prints("Hello, world!"));
}

#[test]
fn complex_paths() {
    given(r#"
        First off:

        ```toml,file=Cargo.toml
        [package]
        authors = ["Pascal Hertleif <killercup@gmail.com>"]
        name = "foo"
        version = "0.1.0"

        [[bin]]
        name = "lolwut"
        path = "src/bin/lolwut/main.rs"
        ```

        And then:

        ```rust,no_run,file=src/bin/lolwut/main.rs
        fn main() {
            println!("Sup dawg I herd u likd nested dirs");
        }
        ```
    "#)
    .waltz()
    .creates(file("Cargo.toml"))
    .creates(file("src/bin/lolwut/main.rs"))
    .cargo_run(|cmd|
        cmd
        .with_args(&["--bin", "lolwut"])
        .prints("Sup dawg")
    );
}
