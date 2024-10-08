mod pb;

use pb::*;
use prost::Message;

fn main() {
    let request = RequestGet {
        key: String::from("hello"),
    };
    let mut buf = Vec::new();
    request.encode(&mut buf).unwrap();
    println!("Hello, world!, encoded: {:?}", buf);
}
