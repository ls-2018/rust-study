use sha1::{Digest, Sha1};


fn main() {
    let hash = Sha1::digest(b"data");
    println!("{:?}", hex::encode(hash));
}
