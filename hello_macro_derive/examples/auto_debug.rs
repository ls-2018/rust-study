#![allow(unused)]

use macros::AutoDebug;
use std::fmt::{Debug, Formatter};

#[derive(AutoDebug)]
pub struct RespBulkString {
    inner: String,
    #[debug(skip)]
    nothing: (),
    hello: u32,
}

fn main() {
    let s = RespBulkString {
        inner: "hello".to_string(),
        nothing: (),
        hello: 42,
    };
    println!("{:?}", s);
}
