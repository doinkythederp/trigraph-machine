# trigraph-machine

This is a CLI tool I made in Rust for replacing [C trigraphs](https://en.wikipedia.org/wiki/Digraphs_and_trigraphs#C) in strings. You can pipe to and from it, like grep.

```console
$ echo "int main() ??<??>" | trigraph
int main() {}
$ echo "// Testing123??/" | trigraph
// Testing123\
```

## Building

Compile the project using `cargo`:

```bash
cargo build --release
cargo build
```
