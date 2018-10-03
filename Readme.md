# Waltz

Extract code files from Markdown files.

Write guides in Markdown with code blocks that belong in several files, and let _waltz_ extract the code for you so you can build/run/test it easily.

## DANGER: Will eat your laundry

This is highly experimental. Has not been tested with real-world code.

## Install binary

There are pre-compiled binaries for newer releases that you can find on the [releases page](https://github.com/killercup/waltz/releases).

Alternatively, install it like this:

```bash
curl -LSfs https://japaric.github.io/trust/install.sh | sh -s -- --git killercup/waltz
```

## Install manually

Make sure you have Rust and Cargo installed.

```bash
$ cargo install --git https://github.com/killercup/waltz
```

Alternatively, clone this repository and run `cargo install --path waltz_cli`.

## Usage

### Markdown syntax

Write regular Markdown, but add an additional file flag to your code blocks, so they have `lang,file=path` after the triple back ticks. (This is the relative path of the file this code should end up in).

Example:

    # Getting started

    First of all, create a simple `Cargo.toml` file:

    ```toml,file=Cargo.toml
    [package]
    authors = ["Pascal Hertleif <killercup@gmail.com>"]
    name = "foo"
    version = "0.1.0"
    ```

### As Rust library

Add `waltz = "0.4"` to your dependencies and use it!

**[API documentation](https://docs.rs/waltz/)**

### CLI

```bash
$ cargo install waltz_cli
$ waltz
[...]
Usage: waltz <input file> [<target directory>]
[...]
$ waltz docs/guides/getting-started.md
```

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
