# hyper-hello-world

Rust Hyper's hello world example with explanations for C devs

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://mit-license.org/)

## Purpose

When I saw Hyper's hello world example, I wondered what all those closures and futures were doing, so I rewrote in a more traditional style and stripped out the syntax sugar. In this repo are four source files, beginning with the original hello world in `base.rs` (taken from the example on https://hyper.rs/) and progressively explicating parts until reaching `full.rs`.

## Building and Running

~~~
# Build all four binaries
cargo build

# Run each binary, from original example to most explicit
cargo run --bin base
cargo run --bin fn
cargo run --bin fn-error
cargo run --bin full

# Check server for response
curl 127.0.0.1:3000
~~~

## TODO

- Can it print the startup message after starting?
- Not-unit test?
