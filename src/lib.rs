//! byte-io: a simple crate for read/write numbers to/from binary.
//!
//! This crate only contains 4 functions:
//!
//! * `write_be`: write number to big-endian slice.
//!
//! * `read_be`: read number from big-endian slice.
//!
//! * `write_le`: write number to little-endian slice.
//!
//! * `read_le`: read number from little-endian slice.
//!
//! ## Examples:
//!
//! Read from a slice is simple:
//!
//! ```
//! use byte_io::*;
//!
//! fn main() {
//!     let data = [0x00, 0x00, 0x01, 0x01, 0xAB, 0xCD, 0xEF, 0x89];
//!     assert_eq!(read_be::<u32>(&data), 0x0101);
//!     assert_eq!(read_be::<u16>(&data[4..]), 0xABCD);
//!     assert_eq!(read_le::<u16>(&data[4..]), 0xCDAB);
//!     assert_eq!(read_le::<u8>(&data[4..]), 0xAB);
//! }
//!
//! ```
//!
//! Write is also easy:
//!
//! ```
//! use byte_io::*;
//!
//! fn main() {
//!     let mut buf = [0u8;8];
//!     write_be(&0xABCDEFu32, &mut buf);
//!     assert_eq!(buf, [0x00, 0xAB, 0xCD, 0xEF, 0x00, 0x00, 0x00, 0x00]);
//!     write_le(&0xABCDEFu32, &mut buf[4..]);
//!     assert_eq!(buf, [0x00, 0xAB, 0xCD, 0xEF, 0xEF, 0xCD, 0xAB, 0x00]);
//! }
//! ```
//!
//! Moreover, you can even read/write `Vec<T>`:
//!
//! ```
//! use byte_io::*;
//!
//! fn main() {
//!     let mut buf = [0u8;8];
//!     let data = vec![0x1234u16,0x5678u16];
//!     write_le(&data, &mut buf);
//!     assert_eq!(buf, [0x34, 0x12, 0x78, 0x56, 0x00, 0x00, 0x00, 0x00]);
//!     assert_eq!(data, read_le::<Vec<u16>>(&buf[0..4]));
//!     let u32_vec = read_be::<Vec<u32>>(&buf[4..]);
//!     assert_eq!(u32_vec.len(), 1);
//!     assert_eq!(u32_vec.first(), Some(&0));
//! }
//! ```
//!
//! The following code also works:
//!
//! ```
//! use byte_io::*;
//!
//! fn main() {
//!     let buf = [0xAA, 0xBB, 0xCC, 0xDD];
//!     assert_eq!(u32::from_u8_be(&buf), 0xAABBCCDD);
//! }
//! ```
//!
//!
//! ## Implementation Details
//!
//! byte-io does __NOT__ focus on efficiency, which means that it may be slow
//! while handling big streams (e.g. hundreds of Mbytes or more).
//!
//! Generally speaking, byte-io implements the two traits for numbers: `Readable` and
//! `Writeable`. Every type implements these two traits can be deceded/enceded from
//! binary stream.
use std::marker;
use std::mem::{size_of, transmute};

/// write a number to stream as big-endian.
///
/// panics if buffer does not contain enough space.
///
/// ```
/// use byte_io::*;
///
/// let mut buf = [0u8;8];
/// write_be(&1u64, &mut buf);
/// assert_eq!(buf, [0,0,0,0,0,0,0,1]);
/// ```
pub fn write_be<T: Writeable>(v: &T, buffer: &mut [u8]) {
    T::to_u8_be(v, buffer);
}

/// read a number from stream as big-endian.
///
/// panics if buffer does not contain enough bytes.
///
/// ```
/// use byte_io::*;
/// let data = [0xAB, 0xCD, 0xEF, 0x01, 0x23];
/// assert_eq!(read_be::<u32>(&data), 0xABCDEF01);
/// assert_eq!(read_be::<i16>(&data[3..]), 0x0123);
/// ```
pub fn read_be<T: Readable>(buffer: &[u8]) -> T {
    T::from_u8_be(buffer)
}

/// write a number to stream as little-endian.
///
/// panics if buffer does not contain enough space.
///
/// ```
/// use byte_io::*;
///
/// let mut buf = [0u8;8];
/// write_le(&1u64, &mut buf);
/// assert_eq!(buf, [1,0,0,0,0,0,0,0]);
/// ```
pub fn write_le<T: Writeable>(v: &T, buffer: &mut [u8]) {
    T::to_u8_le(v, buffer);
}

/// read a number from stream as big-endian.
///
/// panics if buffer does not contain enough bytes.
///
/// ```
/// use byte_io::*;
/// let data = [0xAB, 0xCD, 0xEF, 0x01, 0x23];
/// assert_eq!(read_le::<u32>(&data), 0x01EFCDAB);
/// assert_eq!(read_le::<i16>(&data[3..]), 0x2301);
/// ```
pub fn read_le<T: Readable>(buffer: &[u8]) -> T {
    T::from_u8_le(buffer)
}

/// Any type implementing Readable can be decoded from binary.
pub trait Readable : marker::Sized {
    fn from_u8_be(&[u8]) -> Self;
    fn from_u8_le(&[u8]) -> Self;
}

/// Any type implementing Writeable can be encoded from binary.
pub trait Writeable : marker::Sized {
    fn to_u8_be(&Self, &mut [u8]);
    fn to_u8_le(&Self, &mut [u8]);
}


impl<T: Readable> Readable for Vec<T> {
    fn from_u8_be(input: &[u8]) -> Self {
        let t_size = size_of::<T>();
        let mut output = Vec::new();
        for i in 0..input.len() / t_size {
            output.push(T::from_u8_be(&input[i * t_size..(i + 1) * t_size]));
        }
        output
    }

    fn from_u8_le(input: &[u8]) -> Self {
        let t_size = size_of::<T>();
        let mut output = Vec::new();
        for i in 0..input.len() / t_size {
            output.push(T::from_u8_le(&input[i * t_size..(i + 1) * t_size]));
        }
        output
    }
}

impl<T: Writeable> Writeable for Vec<T> {
    fn to_u8_be(v: &Self, buf: &mut [u8]) {
        let t_size = size_of::<T>();
        for (i, v) in v.iter().enumerate() {
            T::to_u8_be(v, &mut buf[i * t_size..(i + 1) * t_size]);
        }
    }

    fn to_u8_le(v: &Self, buf: &mut [u8]) {
        let t_size = size_of::<T>();
        for (i, v) in v.iter().enumerate() {
            T::to_u8_le(v, &mut buf[i * t_size..(i + 1) * t_size]);
        }
    }
}

impl Writeable for i8 {
    fn to_u8_be(v: &Self, a: &mut [u8]) {
        a[0] = *v as u8;
    }

    fn to_u8_le(v: &Self, a: &mut [u8]) {
        a[0] = *v as u8;
    }
}

impl Readable for i8 {
    fn from_u8_be(i: &[u8]) -> Self {
        i[0] as i8
    }

    fn from_u8_le(i: &[u8]) -> Self {
        i[0] as i8
    }
}

impl Readable for u8 {
    fn from_u8_be(a: &[u8]) -> Self {
        a[0]
    }

    fn from_u8_le(a: &[u8]) -> Self {
        a[0]
    }
}

impl Writeable for u8 {
    fn to_u8_be(v: &Self, a: &mut [u8]) {
        a[0] = *v;
    }

    fn to_u8_le(v: &Self, a: &mut [u8]) {
        a[0] = *v;
    }
}

impl Readable for i16 {
    fn from_u8_be(i: &[u8]) -> Self {
        (i[0] as i16) << 8 | i[1] as i16
    }

    fn from_u8_le(i: &[u8]) -> Self {
        (i[1] as i16) << 8 | i[0] as i16
    }
}

impl Writeable for i16 {
    fn to_u8_be(v: &Self, a: &mut [u8]) {
        a[0] = (*v >> 8) as u8;
        a[1] = *v as u8;
    }

    fn to_u8_le(v: &Self, a: &mut [u8]) {
        a[1] = (*v >> 8) as u8;
        a[0] = *v as u8;
    }
}

impl Readable for u16 {
    fn from_u8_be(i: &[u8]) -> Self {
        (i[0] as u16) << 8 | i[1] as u16
    }

    fn from_u8_le(i: &[u8]) -> Self {
        (i[1] as u16) << 8 | i[0] as u16
    }
}

impl Writeable for u16 {
    fn to_u8_be(v: &Self, a: &mut [u8]) {
        a[0] = (*v >> 8) as u8;
        a[1] = *v as u8;
    }

    fn to_u8_le(v: &Self, a: &mut [u8]) {
        a[1] = (*v >> 8) as u8;
        a[0] = *v as u8;
    }
}

impl Readable for i32 {
    fn from_u8_be(i: &[u8]) -> Self {
        (i[0] as i32) << 24 | (i[1] as i32) << 16 | (i[2] as i32) << 8 | i[3] as i32
    }

    fn from_u8_le(i: &[u8]) -> Self {
        (i[3] as i32) << 24 | (i[2] as i32) << 16 | (i[1] as i32) << 8 | i[0] as i32
    }
}

impl Writeable for i32 {
    fn to_u8_be(v: &Self, a: &mut [u8]) {
        a[0] = (*v >> 24) as u8;
        a[1] = (*v >> 16) as u8;
        a[2] = (*v >> 8) as u8;
        a[3] = *v as u8;
    }

    fn to_u8_le(v: &Self, a: &mut [u8]) {
        a[3] = (*v >> 24) as u8;
        a[2] = (*v >> 16) as u8;
        a[1] = (*v >> 8) as u8;
        a[0] = *v as u8;
    }
}

impl Readable for u32 {
    fn from_u8_be(i: &[u8]) -> Self {
        (i[0] as u32) << 24 | (i[1] as u32) << 16 | (i[2] as u32) << 8 | i[3] as u32
    }

    fn from_u8_le(i: &[u8]) -> Self {
        (i[3] as u32) << 24 | (i[2] as u32) << 16 | (i[1] as u32) << 8 | i[0] as u32
    }
}

impl Writeable for u32 {
    fn to_u8_be(v: &Self, a: &mut [u8]) {
        a[0] = (*v >> 24) as u8;
        a[1] = (*v >> 16) as u8;
        a[2] = (*v >> 8) as u8;
        a[3] = *v as u8;
    }

    fn to_u8_le(v: &Self, a: &mut [u8]) {
        a[3] = (*v >> 24) as u8;
        a[2] = (*v >> 16) as u8;
        a[1] = (*v >> 8) as u8;
        a[0] = *v as u8;
    }
}

impl Readable for i64 {
    fn from_u8_be(i: &[u8]) -> Self {
        (i[0] as i64) << 56 | (i[1] as i64) << 48 | (i[2] as i64) << 40 | (i[3] as i64) << 32 |
        (i[4] as i64) << 24 | (i[5] as i64) << 16 |
        (i[6] as i64) << 8 | i[7] as i64
    }

    fn from_u8_le(i: &[u8]) -> Self {
        (i[7] as i64) << 56 | (i[6] as i64) << 48 | (i[5] as i64) << 40 | (i[4] as i64) << 32 |
        (i[3] as i64) << 24 | (i[2] as i64) << 16 |
        (i[1] as i64) << 8 | i[0] as i64
    }
}

impl Writeable for i64 {
    fn to_u8_be(v: &Self, a: &mut [u8]) {
        a[0] = (*v >> 56) as u8;
        a[1] = (*v >> 48) as u8;
        a[2] = (*v >> 40) as u8;
        a[3] = (*v >> 32) as u8;
        a[4] = (*v >> 24) as u8;
        a[5] = (*v >> 16) as u8;
        a[6] = (*v >> 8) as u8;
        a[7] = *v as u8;
    }

    fn to_u8_le(v: &Self, a: &mut [u8]) {
        a[7] = (*v >> 56) as u8;
        a[6] = (*v >> 48) as u8;
        a[5] = (*v >> 40) as u8;
        a[4] = (*v >> 32) as u8;
        a[3] = (*v >> 24) as u8;
        a[2] = (*v >> 16) as u8;
        a[1] = (*v >> 8) as u8;
        a[0] = *v as u8;
    }
}

impl Readable for u64 {
    fn from_u8_be(i: &[u8]) -> Self {
        (i[0] as u64) << 56 | (i[1] as u64) << 48 | (i[2] as u64) << 40 | (i[3] as u64) << 32 |
        (i[4] as u64) << 24 | (i[5] as u64) << 16 |
        (i[6] as u64) << 8 | i[7] as u64
    }

    fn from_u8_le(i: &[u8]) -> Self {
        (i[7] as u64) << 56 | (i[6] as u64) << 48 | (i[5] as u64) << 40 | (i[4] as u64) << 32 |
        (i[3] as u64) << 24 | (i[2] as u64) << 16 |
        (i[1] as u64) << 8 | i[0] as u64
    }
}

impl Writeable for u64 {
    fn to_u8_be(v: &Self, a: &mut [u8]) {
        a[0] = (*v >> 56) as u8;
        a[1] = (*v >> 48) as u8;
        a[2] = (*v >> 40) as u8;
        a[3] = (*v >> 32) as u8;
        a[4] = (*v >> 24) as u8;
        a[5] = (*v >> 16) as u8;
        a[6] = (*v >> 8) as u8;
        a[7] = *v as u8;
    }

    fn to_u8_le(v: &Self, a: &mut [u8]) {
        a[7] = (*v >> 56) as u8;
        a[6] = (*v >> 48) as u8;
        a[5] = (*v >> 40) as u8;
        a[4] = (*v >> 32) as u8;
        a[3] = (*v >> 24) as u8;
        a[2] = (*v >> 16) as u8;
        a[1] = (*v >> 8) as u8;
        a[0] = *v as u8;
    }
}

impl Readable for bool {
    fn from_u8_be(i: &[u8]) -> Self {
        i[0] > 0
    }

    fn from_u8_le(i: &[u8]) -> Self {
        i[0] > 0
    }
}

impl Writeable for bool {
    fn to_u8_be(v: &Self, a: &mut [u8]) {
        a[0] = if *v {
            1u8
        } else {
            0u8
        };
    }

    fn to_u8_le(v: &Self, a: &mut [u8]) {
        a[0] = if *v {
            1u8
        } else {
            0u8
        };
    }
}

impl Readable for f32 {
    fn from_u8_be(i: &[u8]) -> Self {
        unsafe { transmute(u32::from_u8_be(i)) }
    }

    fn from_u8_le(i: &[u8]) -> Self {
        unsafe { transmute(u32::from_u8_le(i)) }
    }
}

impl Writeable for f32 {
    fn to_u8_be(v: &Self, a: &mut [u8]) {
        unsafe { u32::to_u8_be(transmute(v), a) }
    }

    fn to_u8_le(v: &Self, a: &mut [u8]) {
        unsafe { u32::to_u8_le(transmute(v), a) }
    }
}

impl Readable for f64 {
    fn from_u8_be(i: &[u8]) -> Self {
        unsafe { transmute(u64::from_u8_be(i)) }
    }

    fn from_u8_le(i: &[u8]) -> Self {
        unsafe { transmute(u64::from_u8_le(i)) }
    }
}

impl Writeable for f64 {
    fn to_u8_be(v: &Self, a: &mut [u8]) {
        unsafe { u64::to_u8_be(transmute(v), a) }
    }

    fn to_u8_le(v: &Self, a: &mut [u8]) {
        unsafe { u64::to_u8_le(transmute(v), a) }
    }
}
