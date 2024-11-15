#![allow(unused)]
use macros::AutoDeref;

#[derive(Debug, AutoDeref)]
#[deref(field = "inner", mutable = true)]
#[deref2(a = true, b = true)]
pub struct RespBulkString {
    inner: String,
    nothing: (),
}

fn main() {
    let s = RespBulkString {
        inner: "hello".to_string(),
        nothing: (),
    };
    println!("{:?}", s);
}
