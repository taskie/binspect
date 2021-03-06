# binspect

[![Test](https://github.com/taskie/binspect/workflows/Test/badge.svg)](https://github.com/taskie/binspect/actions)
[![Latest version](https://img.shields.io/crates/v/binspect.svg)](https://crates.io/crates/binspect)
[![Documentation](https://docs.rs/binspect/badge.svg)](https://docs.rs/binspect)
![License](https://img.shields.io/crates/l/binspect.svg)

Rust utilities to inspect the data layout of objects.

This library is for debugging only because data layout of Rust is not be stabilized.
Please read [Data Layout - The Rustonomicon](https://doc.rust-lang.org/stable/nomicon/data.html) in detail.

## Usage

```rust
use binspect::binspect;

let s = "ABC";
binspect!(s);
binspect!(*s);
```

An example of output (depends on compilation and runtime environments):

```text
-----+ 0x7ffce3c8f7a0: &str = s
0000 | 49 03 b4 2f 2c 56 00 00 : 03 00 00 00 00 00 00 00
-----+ 0x562c2fb40349: str = *s
0000 | 41 42 43
```

## Examples

See [examples.md](examples.md) and [its original source](examples/all.rs).

## License

MIT or Apache-2.0
