/*!
Utilities to inspect the data layout of objects.

This library is for debugging only because data layout of Rust is not be stabilized.
Please read [Data Layout - The Rustonomicon](https://doc.rust-lang.org/stable/nomicon/data.html) in detail.

# Examples

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
*/

use std::any::type_name;
use std::io::{self, Write};
use std::mem;
use std::ptr;

#[inline]
#[doc(hidden)]
pub unsafe fn as_bytes_with_len<T: ?Sized>(t: &T, len: usize) -> &[u8] {
    let p = t as *const _ as *const u8;
    &*ptr::slice_from_raw_parts(p, len)
}

#[inline]
#[doc(hidden)]
pub fn as_bytes<T: ?Sized>(t: &T) -> &[u8] {
    unsafe { as_bytes_with_len(t, mem::size_of_val::<T>(t)) }
}

#[doc(hidden)]
pub struct Record<'a, T: ?Sized> {
    pub reference: &'a T,
    pub bytes: &'a [u8],
    pub sized: bool,
    pub source: &'a str,
    pub label: Option<&'a str>,
    pub file: &'a str,
    pub line: u32,
    pub column: u32,
}

#[doc(hidden)]
pub fn write_internal<W: Write, T: ?Sized>(
    mut w: W,
    record: &Record<T>,
    absolute: bool,
) -> Result<(), io::Error> {
    let width = 16;
    let center = width / 2;
    if absolute {
        writeln!(
            w,
            "{:p} : {} = {}",
            record.reference,
            type_name::<T>(),
            record.source
        )?;
    } else {
        writeln!(
            w,
            "-----+ {:p}: {} = {}",
            record.reference,
            type_name::<T>(),
            record.source
        )?;
    }
    for (i, x) in record.bytes.iter().enumerate() {
        if i % width == 0 {
            if i != 0 {
                writeln!(w)?;
            }
            if absolute {
                write!(w, "{:p} |", unsafe {
                    (record.reference as *const _ as *const u8).add(i)
                })?;
            } else {
                write!(w, "{:04x} |", i)?;
            }
        } else if i % center == 0 {
            write!(w, " :")?;
        }
        write!(w, " {:02x}", x)?;
    }
    if !record.bytes.is_empty() {
        writeln!(w)?;
    }
    Ok(())
}

#[inline]
#[doc(hidden)]
pub fn print_internal<T: ?Sized>(record: &Record<T>, absolute: bool) {
    write_internal(io::stdout().lock(), record, absolute).unwrap()
}

#[inline]
#[doc(hidden)]
pub fn eprint_internal<T: ?Sized>(record: &Record<T>, absolute: bool) {
    write_internal(io::stderr().lock(), record, absolute).unwrap()
}

#[macro_export]
#[doc(hidden)]
macro_rules! record {
    ($t: expr, $v: expr, $bs: expr, $sized: expr) => {{
        let bytes = $bs;
        $crate::Record {
            reference: $t,
            bytes,
            sized: $sized,
            source: stringify!($v),
            label: None,
            file: file!(),
            line: line!(),
            column: column!(),
        }
    }};
}

/// Prints the memory address and the hex representation of an object to stdout.
///
/// # Examples
///
/// ```
/// # use binspect::binspect;
/// let s = "ABC";
/// binspect!(s);
/// binspect!(*s);
/// ```
#[macro_export]
macro_rules! binspect {
    ($v: expr) => {{
        let t = &$v;
        let bs = $crate::as_bytes(t);
        $crate::print_internal(&$crate::record!(t, $v, bs, true), false);
    }};
    ($v: expr, $len: expr) => {{
        let t = &$v;
        let bs = $crate::as_bytes_with_len(t, $len);
        $crate::print_internal(&$crate::record!(t, $v, bs, false), false);
    }};
}

/// Prints the memory address and the hex representation of an object to stderr.
///
/// # Examples
///
/// ```
/// # use binspect::ebinspect;
/// let s = "ABC";
/// ebinspect!(s);
/// ebinspect!(*s);
/// ```
#[macro_export]
macro_rules! ebinspect {
    ($v: expr) => {{
        let t = &$v;
        let bs = $crate::as_bytes(t);
        $crate::eprint_internal(&$crate::record!(t, $v, bs, true), false);
    }};
    ($v: expr, $len: expr) => {{
        let t = &$v;
        let bs = $crate::as_bytes_with_len(t, $len);
        $crate::eprint_internal(&$crate::record!(t, $v, bs, false), false);
    }};
}

/// Writes the memory address and the hex representation of an object to [`std::io::Write`].
///
/// # Examples
///
/// ```
/// # use binspect::write_binspect;
/// let s = "ABC";
/// let mut buf: Vec<u8> = vec![];
/// write_binspect!(&mut buf, s).unwrap();
/// buf.clear();
/// write_binspect!(&mut buf, *s).unwrap();
/// ```
#[macro_export]
macro_rules! write_binspect {
    ($w: expr, $v: expr) => {{
        let t = &$v;
        let bs = $crate::as_bytes(t);
        $crate::write_internal($w, &$crate::record!(t, $v, bs, true), false)
    }};
    ($w: expr, $v: expr, $len: expr) => {{
        let t = &$v;
        let bs = $crate::as_bytes_with_len(t, $len);
        $crate::write_internal($w, &$crate::record!(t, $v, bs, false), false)
    }};
}
