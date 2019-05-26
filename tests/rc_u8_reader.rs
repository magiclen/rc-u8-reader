extern crate rc_u8_reader;

use std::rc::Rc;
use std::io::Read;

use rc_u8_reader::RcU8Reader;

#[test]
fn to_string() {
    let data = b"Hello world!".to_vec();

    let mut reader = RcU8Reader::new(Rc::new(data));

    let mut result = String::new();

    reader.read_to_string(&mut result).unwrap();

    assert_eq!("Hello world!".to_string(), result);
}