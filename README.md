![Screenshot of `gamels` on a Steam Deck](steamdeck-screenshot.png)
# gamels
[![Crates.io](https://img.shields.io/crates/v/gamels)](https://crates.io/crates/gamels) 
[![Build](https://github.com/Ewpratten/gamels/actions/workflows/build.yml/badge.svg)](https://github.com/Ewpratten/gamels/actions/workflows/build.yml)
[![Clippy](https://github.com/Ewpratten/gamels/actions/workflows/clippy.yml/badge.svg)](https://github.com/Ewpratten/gamels/actions/workflows/clippy.yml)


`gamels` will list all files in a directory in the style of `ls -la`, but will also show the name of any Steam game found in the listing. This utility was made to help me explore the filesystem of the Steam Deck. 

## Installation

This crate can be installed via `cargo` with:

```sh
cargo install gamels
```

It may also be built for the Steam Deck with:

```sh
cargo build --release --target x86_64-unknown-linux-musl
```
