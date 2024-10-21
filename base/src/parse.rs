#[test]

fn main() {
    let a = "10".parse::<u32>();

    let aa: u32 = "10".parse().unwrap(); // 这种写法也很常见
    println!("{:?}", a);

    let a = "10".parse::<f32>();

    println!("{:?}", a);

    let a = "4.2".parse::<f32>();

    println!("{:?}", a);

    let a = "true".parse::<bool>();

    println!("{:?}", a);

    let a = "a".parse::<char>();

    println!("{:?}", a);

    let a = "192.168.1.100".parse::<std::net::IpAddr>();

    println!("{:?}", a);
}
