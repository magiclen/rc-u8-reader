use std::{
    io::Read,
    sync::{Arc, Mutex},
};

use rc_u8_reader::ArcMutexU8Reader;

#[test]
fn to_string() {
    let data = b"Hello world!".to_vec();

    let mut reader = ArcMutexU8Reader::new(Arc::new(Mutex::new(data)));

    let mut result = String::new();

    reader.read_to_string(&mut result).unwrap();

    assert_eq!("Hello world!".to_string(), result);
}
