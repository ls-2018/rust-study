use sha1::{Digest, Sha1};

#[test]
fn t() {
    let hash = Sha1::digest(b"data");
    println!("{:?}", hex::encode(hash));
}
