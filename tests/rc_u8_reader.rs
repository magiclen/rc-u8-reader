use std::{io::Read, rc::Rc};

use rc_u8_reader::RcU8Reader;

#[test]
fn to_string() {
    let data: Rc<str> = "Hello world!".into();

    let mut reader = RcU8Reader::new(data);

    let mut result = String::new();

    reader.read_to_string(&mut result).unwrap();

    assert_eq!("Hello world!".to_string(), result);
}
