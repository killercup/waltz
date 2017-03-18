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
