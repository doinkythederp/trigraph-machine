# trigraph-machine

This is a CLI tool I made for replacing [C trigraphs](https://en.wikipedia.org/wiki/Digraphs_and_trigraphs#C) in strings. You can pipe to and from it, like grep.

```console
$ echo "int main() ??<??>" | trigraph
int main() {}
$ echo "// Testing123??/" | trigraph
// Testing123\
```

- Written in Rust :crab:
- I made it in a few hours :alarm_clock:
- Has a few unit tests :white_check_mark:

## Building

Compile the project using `cargo`:

```bash
cargo build --release
cargo build
```
