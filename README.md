# calc

A very simple command-line calculator written in Rust.

[`meval`](https://crates.io/crates/meval) does all the heavy lifting.

This is my first Rust program!

## Build & Usage

To build:

```sh
cargo build --release
```

To use, pass an expression as a command-line argument:

```sh
calc "tan(pi/4)*10^2"
100
```
(the calc binary will be located in calc/target/release)
