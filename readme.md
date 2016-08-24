## byte-io: a simple crate for read/write numbers to/from binary.

This crate only contains 4 functions:

* `write_be`: write number to big-endian slice.

* `read_be`: read number from big-endian slice.

* `write_le`: write number to little-endian slice. 

* `read_le`: read number from little-endian slice.

Please notice that byte-io does __NOT__ focus on efficiency, which means that it may be slow
while handling big streams (e.g. hundreds of Mbytes or more). 

## How to use

Add the following line to your `[dependencies]` section in `Cargo.toml`

```toml
byte-io = { git = "https://github.com/zhaihj/byte-io-rust", branch= "master" }
```

or you can also download it from [crates.io](http://crates.io):

```toml
byte-io = "0.1"
```

## Examples:

Read from a slice is simple:

```rust
use byte_io::*;

fn main() {
 let data = [0x00, 0x00, 0x01, 0x01, 0xAB, 0xCD, 0xEF, 0x89];
 assert_eq!(read_be::<u32>(&data), 0x0101);
 assert_eq!(read_be::<u16>(&data[4..]), 0xABCD);
 assert_eq!(read_le::<u16>(&data[4..]), 0xCDAB);
 assert_eq!(read_le::<u8>(&data[4..]), 0xAB);
}

```

Write is also easy:

```rust
use byte_io::*;

fn main() {
 let mut buf = [0u8;8];
 write_be(&0xABCDEFu32, &mut buf);
 assert_eq!(buf, [0x00, 0xAB, 0xCD, 0xEF, 0x00, 0x00, 0x00, 0x00]);
 write_le(&0xABCDEFu32, &mut buf[4..]);
 assert_eq!(buf, [0x00, 0xAB, 0xCD, 0xEF, 0xEF, 0xCD, 0xAB, 0x00]);
}
```

Moreover, you can even read/write `Vec<T>`:

```rust
use byte_io::*;

fn main() {
 let mut buf = [0u8;8];
 let data = vec![0x1234u16,0x5678u16];
 write_le(&data, &mut buf);
 assert_eq!(buf, [0x34, 0x12, 0x78, 0x56, 0x00, 0x00, 0x00, 0x00]);
 assert_eq!(data, read_le::<Vec<u16>>(&buf[0..4]));
 let u32_vec = read_be::<Vec<u32>>(&buf[4..]);
 assert_eq!(u32_vec.len(), 1);
 assert_eq!(u32_vec.first(), Some(&0));
}
```

The following code also works:

```rust
use byte_io::*;

fn main() {
 let buf = [0xAA, 0xBB, 0xCC, 0xDD];
 assert_eq!(u32::from_u8_be(&buf), 0xAABBCCDD);
}
```
