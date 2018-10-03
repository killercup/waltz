mod utils;
use utils::*;

#[test]
fn concat() {
    given(
        r#"
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

        Very good. Let us implement a `new` function:

        ```rust,file=src/lib.rs
        impl Foo {
            fn new(x: i32) -> Foo {
                Foo { x: x }
            }
        }
        ```

        Great. How about a good default as well?

        ```rust,file=src/lib.rs
        use std::default::Default;

        impl Default for Foo {
            fn default() -> Foo {
                Foo { x: 42 }
            }
        }
        ```
    "#,
    ).running(waltz)
    .creates(file("Cargo.toml").containing(
        r#"
        [package]
        authors = ["Pascal Hertleif <killercup@gmail.com>"]
        name = "foo"
        version = "0.1.0"
    "#,
    )).creates(file("src/lib.rs").containing(
        r#"
        struct Foo {
            x: i32,
        }
        impl Foo {
            fn new(x: i32) -> Foo {
                Foo { x: x }
            }
        }
        use std::default::Default;

        impl Default for Foo {
            fn default() -> Foo {
                Foo { x: 42 }
            }
        }
    "#,
    )).running(cargo_check);
}
