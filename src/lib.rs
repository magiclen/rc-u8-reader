/*!
# Rc U8 Reader

A tiny implement for reading `u8` data from a reference counted instance.

## Examples

### RcU8Reader

```rust
extern crate rc_u8_reader;

use std::rc::Rc;
use std::io::Read;

use rc_u8_reader::RcU8Reader;

let data = b"Hello world!".to_vec();

let mut reader = RcU8Reader::new(Rc::new(data));

let mut result = String::new();

reader.read_to_string(&mut result).unwrap();

assert_eq!("Hello world!".to_string(), result);
```

### RcRefCellU8Reader

```rust
extern crate rc_u8_reader;

use std::cell::RefCell;
use std::rc::Rc;
use std::io::Read;

use rc_u8_reader::RcRefCellU8Reader;

let data = b"Hello world!".to_vec();

let mut reader = RcRefCellU8Reader::new(Rc::new(RefCell::new(data)));

let mut result = String::new();

reader.read_to_string(&mut result).unwrap();

assert_eq!("Hello world!".to_string(), result);
```
*/

#![cfg_attr(feature = "nightly", allow(stable_features), feature(read_initializer, seek_convenience, iovec))]

mod rc_u8_reader;
mod rc_refcell_u8_reader;

pub use self::rc_u8_reader::RcU8Reader;
pub use rc_refcell_u8_reader::RcRefCellU8Reader;