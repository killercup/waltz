# Waltz

Extract code files from Markdown files.

Write guides in Markdown with code blocks that belong in several files, and let _waltz_ extract the code for you so you can build/run/test it easily.

Meant as a companion to [tango].

[tango]: https://github.com/pnkfelix/tango

## DANGER: Will eat you laundry

This is highly experimental. Has not been tested with real-world code.

## Install

Make sure you have Rust and Cargo installed.

```bash
$ cargo install --git https://github.com/killercup/waltz
```

## Usage

### Markdown syntax

Write regular Markdown, but add an addional file flag to your code blocks, so they have `lang,file=path` after the tripple back ticks. (This is the relative path of the file this code should end up in).

Example:

    # Getting started

    First of all, create a simple `Cargo.toml` file:

    ```toml,file=Cargo.toml
    [package]
    authors = ["Pascal Hertleif <killercup@gmail.com>"]
    name = "foo"
    version = "0.1.0"
    ```

### CLI

```bash
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