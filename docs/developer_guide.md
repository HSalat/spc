# Developer guide

## Code hygiene

We use automated tools to format the code.

```shell
cargo fmt

# Format Markdown docs
prettier --write *.md docs/*.md
```

Install [prettier](https://prettier.io) for Markdown.

## Some tips for working with Rust

There are two equivalent ways to rebuild and then run the code. First:

```shell
cargo run --release -- devon
```

The `--` separates arguments to `cargo`, the Rust build tool, and arguments to
the program itself. The second way:

```shell
cargo build --release
./target/release/aspics devon
```

You can build the code in two ways -- **debug** and **release**. There's a
simple tradeoff -- debug mode is fast to build, but slow to run. Release mode is
slow to build, but fast to run. For the ASPICS codebase, since the input data is
so large and the codebase so small, I'd recommend always using `--release`. If
you want to use debug mode, just omit the flag.

If you're working on the Rust code outside of an IDE like
[VSCode](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust),
then you can check if the code compiles much faster by doing `cargo check`.

## Working with protocol buffers

These instructions will be reorganized. For now, just for reference:

```shell
pip install protobuf

# Regenerate the Python bindings
protoc --python_out=protobuf_samples/ synthpop.proto

# Transform a proto to JSON
python protobuf_samples/protobuf_to_json.py data/output/west_yorkshire_small.pb
```
