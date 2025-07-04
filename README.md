# Irontide

Irontide is an experimental attempt at creating a Rust port of the [Newsboat](https://github.com/newsboat/newsboat) RSS/Atom feed reader. The goal of this project is to provide the same features as Newsboat while taking advantage of Rust's safety and performance.

## Building

This repository uses [Cargo](https://doc.rust-lang.org/cargo/) and requires a stable Rust toolchain. To compile the project run:

```bash
cargo build
```

The project is still in a very early stage but should compile with the stable Rust toolchain.

## Usage

Irontide can read a list of feed URLs from a file and print their titles:

```bash
irontide --url-file urls.txt
```

Each non-empty, non-comment line in `urls.txt` should contain a valid feed URL.

## License

The code is released under the terms of the CC0 license. See the [LICENSE](LICENSE) file for details.
