/*!
Utilities to inspect the data layout of objects.

This library is for debugging only because data layout of Rust is not be stabilized.
Please read [Data Layout - The Rustonomicon](https://doc.rust-lang.org/stable/nomicon/data.html) in detail.

# Examples

```rust
use binspect::binspect;

let s = "ABC";
binspect!(s);
unsafe { binspect!(*s, s.len()) };
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
pub fn as_bytes<T>(t: &T) -> &[u8] {
    unsafe { as_bytes_with_len(t, mem::size_of::<T>()) }
}

#[doc(hidden)]
pub fn write_internal<W: Write, T: ?Sized>(
    mut w: W,
    t: &T,
    bytes: &[u8],
    repr: &str,
    absolute: bool,
) -> Result<(), io::Error> {
    let width = 16;
    let center = width / 2;
    if absolute {
        writeln!(w, "{:p} : {} = {}", t, type_name::<T>(), repr)?;
    } else {
        writeln!(w, "-----+ {:p}: {} = {}", t, type_name::<T>(), repr,)?;
    }
    for (i, x) in bytes.iter().enumerate() {
        if i % width == 0 {
            if i != 0 {
                writeln!(w)?;
            }
            if absolute {
                write!(w, "{:p} |", unsafe { (t as *const _ as *const u8).add(i) })?;
            } else {
                write!(w, "{:04x} |", i)?;
            }
        } else if i % center == 0 {
            write!(w, " :")?;
        }
        write!(w, " {:02x}", x)?;
    }
    if !bytes.is_empty() {
        writeln!(w)?;
    }
    Ok(())
}

#[inline]
#[doc(hidden)]
pub fn print_internal<T: ?Sized>(t: &T, bytes: &[u8], repr: &str, absolute: bool) {
    write_internal(io::stdout().lock(), t, bytes, repr, absolute).unwrap()
}

#[inline]
#[doc(hidden)]
pub fn eprint_internal<T: ?Sized>(t: &T, bytes: &[u8], repr: &str, absolute: bool) {
    write_internal(io::stderr().lock(), t, bytes, repr, absolute).unwrap()
}

/// Prints the memory address and the hex representation of an object to stdout.
///
/// # Examples
///
/// ```
/// # use binspect::binspect;
/// let s = "ABC";
/// binspect!(s);
/// unsafe { binspect!(*s, s.len()) };
/// ```
///
/// # Safety
///
/// A correct byte length must be specified if the value is `!Sized`.
#[macro_export]
macro_rules! binspect {
    ($v: expr) => {{
        let t = &$v;
        $crate::print_internal(t, $crate::as_bytes(t), stringify!($v), false);
    }};
    ($v: expr, $len: expr) => {{
        let t = &$v;
        $crate::print_internal(t, $crate::as_bytes_with_len(t, $len), stringify!($v), false);
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
/// unsafe { ebinspect!(*s, s.len()) };
/// ```
///
/// # Safety
///
/// A correct byte length must be specified if the value is `!Sized`.
#[macro_export]
macro_rules! ebinspect {
    ($v: expr) => {{
        let t = &$v;
        $crate::eprint_internal(t, $crate::as_bytes(t), stringify!($v), false);
    }};
    ($v: expr, $len: expr) => {{
        let t = &$v;
        $crate::eprint_internal(t, $crate::as_bytes_with_len(t, $len), stringify!($v), false);
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
/// unsafe { write_binspect!(&mut buf, *s, s.len()) }.unwrap();
/// ```
///
/// # Safety
///
/// A correct byte length must be specified if the value is `!Sized`.
#[macro_export]
macro_rules! write_binspect {
    ($w: expr, $v: expr) => {{
        let t = &$v;
        $crate::write_internal($w, t, $crate::as_bytes(t), stringify!($v), false)
    }};
    ($w: expr, $v: expr, $len: expr) => {{
        let t = &$v;
        $crate::write_internal(
            $w,
            t,
            $crate::as_bytes_with_len(t, $len),
            stringify!($v),
            false,
        )
    }};
}
