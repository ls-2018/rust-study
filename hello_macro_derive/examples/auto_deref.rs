#![allow(unused)]

use macros::AutoDeref;
use std::ops::Deref;

#[derive(Debug, AutoDeref)]
#[deref(field = "inner", mutable = true)]
// #[deref(mutable = true)]
#[deref2(b = true)]
pub struct RespBulkString {
    inner: String,
    // nothing: (),
}

fn main() {
    let s = RespBulkString { inner: "hello".to_string() };
    println!("{:?}", s.deref());
}
