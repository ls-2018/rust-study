#![allow(unused_variables)]
#![allow(dead_code)]

use anyhow::Result;
use bytes::{BufMut, BytesMut};

fn main() -> Result<()> {
    let mut buf = BytesMut::with_capacity(1024);
    buf.extend_from_slice(b"hello world\n");
    let c = b"hello world\n";
    let c = &b"goodbye world"[..];
    buf.put(&b"goodbye world"[..]);
    buf.put_i64(0xdeadbeef);

    println!("{:?}", buf);
    let a = buf.split(); // buf 变成空的了
    let mut b = a.freeze(); // inner data cannot be changed

    let c = b.split_to(12);
    println!("{:?}", c);
    println!("{:?}", b);
    println!("{:?}", buf);

    Ok(())
}

#[cfg(test)]
pub mod tests {
    #[test]
    pub fn entry() {
        super::main().expect("TODO: panic message");
    }
}
