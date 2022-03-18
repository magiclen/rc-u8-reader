use std::io::Read;
use std::sync::Arc;

use rc_u8_reader::ArcU8Reader;

#[test]
fn to_string() {
    let data: Arc<str> = "Hello world!".into();

    let mut reader = ArcU8Reader::new(data);

    let mut result = String::new();

    reader.read_to_string(&mut result).unwrap();

    assert_eq!("Hello world!".to_string(), result);
}
