use std::{cell::RefCell, io::Read, rc::Rc};

use rc_u8_reader::RcRefCellU8Reader;

#[test]
fn to_string() {
    let data = b"Hello world!".to_vec();

    let mut reader = RcRefCellU8Reader::new(Rc::new(RefCell::new(data)));

    let mut result = String::new();

    reader.read_to_string(&mut result).unwrap();

    assert_eq!("Hello world!".to_string(), result);
}
