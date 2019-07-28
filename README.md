Rc U8 Reader
====================

[![Build Status](https://travis-ci.org/magiclen/rc-u8-reader.svg?branch=master)](https://travis-ci.org/magiclen/rc-u8-reader)

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

## Crates.io

https://crates.io/crates/rc-u8-reader

## Documentation

https://docs.rs/rc-u8-reader

## License

[MIT](LICENSE)