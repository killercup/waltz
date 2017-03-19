include!("_utils.rs");

#[test]
fn simple() {
    let (_tmpdir, output_dir) = do_the_waltz!(r#"
        # Getting started

        First of all, create a simple `Cargo.toml` file:

        ```toml,file=Cargo.toml
        [package]
        authors = ["Pascal Hertleif <killercup@gmail.com>"]
        name = "foo"
        version = "0.1.0"
        ```

        Nice. Now you can put this into `src/lib.rs`:

        ```rust,no_run,file=src/lib.rs
        fn main() {
            println!("Hello, world!");
        }
        ```
    "#);

    assert_file!(output_dir.join("Cargo.toml") => r#"
        [package]
        authors = ["Pascal Hertleif <killercup@gmail.com>"]
        name = "foo"
        version = "0.1.0"
    "#);

    assert_file!(output_dir.join("src/lib.rs") => r#"
        fn main() {
            println!("Hello, world!");
        }
    "#);
}
