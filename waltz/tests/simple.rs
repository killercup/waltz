include!("_utils.rs");

assert_files_generated!(simple: r#"
    # Lorem ipsum

    ## Shell

    ```bash
    $ echo "yeah!"
    ```

    ## A Rust example

    ```rust,file=src/lib.rs
    fn main() {
        println!("Dolor sit amet");
    }
    ```
    "# => [
        "src/lib.rs" => r#"
            fn main() {
                println!("Dolor sit amet");
            }
        "#
    ]
);
