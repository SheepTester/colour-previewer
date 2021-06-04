# Colour previewer

An HTTP server that previews colours given their hex values.

It's made in [Rust](https://www.rust-lang.org/). Install it, then do

```sh
cargo run
```

This'll start a local HTTP server on port 3030, which you can find at http://localhost:3030/.

You can find:

- (for experimentation) `/hi/<int>` will respond with `here is cool int: <int>`

- (for experimentation) `/woink/<text>` will respond with `<text>` only if it's 3 UTF-8 code units (≈ 3 characters) long

- `/colour/<hex>` will respond with an image that is entirely just the given colour `<hex>`

There are a few command line arguments that you can use. `--help` lists them.

```sh
cargo run -- --help
```
