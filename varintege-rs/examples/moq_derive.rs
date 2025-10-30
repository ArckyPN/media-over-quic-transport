use varint_derive::VarInt;
use varintege_rs::x;

#[derive(Debug, VarInt)]
struct Message {
    num: x!(i),
}

fn main() {
    let msg = Message { num: 8u8.into() };
    println!("Hello, {msg:?}!")
}
