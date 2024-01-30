# ansi-converter

This is a rust crate and application which can convert an ANSI document from RGB or 256 colors down to 16 colors.

## Usage

```sh
cargo build
./target/debug/ansi-converter input.ansi > output.ansi
```

## Rust

To use as a rust crate:

```rust
use ansi_converter::convert_to_ansi_16;

fn main() {
    let input = "\x1b[48;2;255;255;255mHello, World!";
    let output = convert_to_ansi_16(input);
    assert_eq!(input, "\x1b[97mHello, World!");
}
```
