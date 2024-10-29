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

    let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];

    let seek = 13;
    assert_eq!(s.binary_search_by(|probe| probe.cmp(&seek)), Ok(9));
    let seek = 4;
    assert_eq!(s.binary_search_by(|probe| probe.cmp(&seek)), Err(7));
    let seek = 100;
    assert_eq!(s.binary_search_by(|probe| probe.cmp(&seek)), Err(13));
    let seek = 1;
    let r = s.binary_search_by(|probe| probe.cmp(&seek));
    assert!(match r {
        Ok(1..=4) => true,
        _ => false,
    });

    Ok(())
}
