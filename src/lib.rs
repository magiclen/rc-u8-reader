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

### ArcU8Reader

```rust
extern crate rc_u8_reader;

use std::sync::Arc;
use std::io::Read;

use rc_u8_reader::ArcU8Reader;

let data = b"Hello world!".to_vec();

let mut reader = ArcU8Reader::new(Arc::new(data));

let mut result = String::new();

reader.read_to_string(&mut result).unwrap();

assert_eq!("Hello world!".to_string(), result);
```

### ArcMutexU8Reader

```rust
extern crate rc_u8_reader;

use std::sync::{Arc, Mutex};
use std::io::Read;

use rc_u8_reader::ArcMutexU8Reader;

let data = b"Hello world!".to_vec();

let mut reader = ArcMutexU8Reader::new(Arc::new(Mutex::new(data)));

let mut result = String::new();

reader.read_to_string(&mut result).unwrap();

assert_eq!("Hello world!".to_string(), result);
```
*/

#![cfg_attr(feature = "nightly", feature(read_initializer, seek_convenience))]

#[macro_use]
extern crate debug_helper;

mod rc_refcell_u8_reader;
mod rc_u8_reader;

mod arc_mutex_u8_reader;
mod arc_u8_reader;

pub use self::rc_u8_reader::RcU8Reader;
pub use rc_refcell_u8_reader::RcRefCellU8Reader;

pub use arc_mutex_u8_reader::ArcMutexU8Reader;
pub use arc_u8_reader::ArcU8Reader;
