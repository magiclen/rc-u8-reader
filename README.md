Rc U8 Reader
====================

[![Build Status](https://travis-ci.org/magiclen/rc-u8-reader.svg?branch=master)](https://travis-ci.org/magiclen/rc-u8-reader)
[![Build status](https://ci.appveyor.com/api/projects/status/4pahg84urfpyls8a/branch/master?svg=true)](https://ci.appveyor.com/project/magiclen/rc-u8-reader/branch/master)

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

## Crates.io

https://crates.io/crates/rc-u8-reader

## Documentation

https://docs.rs/rc-u8-reader

## License

[MIT](LICENSE)