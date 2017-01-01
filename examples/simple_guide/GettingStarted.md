# Getting started

First of all, create a simple `Cargo.toml` file:

<figure>
<figcaption>Cargo.toml</figcaption>

```toml
[package]
authors = ["Pascal Hertleif <killercup@gmail.com>"]
name = "foo"
version = "0.1.0"
```

</figure>

Nice. Now you can put this into `src/lib.rs`:

<figure>
<figcaption>src/lib.rs</figcaption>

```rust
fn main() {
  println!("Hello, world!");
}
```

</figure>
