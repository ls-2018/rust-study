use std::collections::HashMap;
use std::io;
use std::net::TcpListener;
use std::thread::spawn;
// #[macro_use]
// extern crate garden;
// Box：指向堆中值的拥有型指针
// Box: <Attend>

fn build_vector() -> Vec<i16> {
    let mut v: Vec<i16> = Vec::<i16>::new();
    v.push(10i16);
    v.push(20i16);
    v
}

/// 不断接受连接，为每个连接启动一个线程
fn echo_main(addr: &str) -> io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    println!("listening on {}", addr);
    loop {
        // 等待客户端连入
        let (mut stream, addr) = listener.accept()?;
        println!("connection received from {}", addr);

        // 启动一个线程来处理此客户端
        let mut write_stream = stream.try_clone()?;
        // move 关键字用于函数参数的生命周期注释，它表示这个参数获得了其引用的数据的所有权。
        // 当你将一个值传递给一个使用 move 关键字的闭包或函数时，该值的所有权会被移动到闭包或函数中，
        // 这意味着原始值不再有效。
        spawn(move || {
            // 回显从`stream`中收到的一切
            io::copy(&mut stream, &mut write_stream).expect("error in client thread: ");
            println!("connection closed");
        });
    }
}

#[allow(dead_code)]
fn main() {
    build_vector();

    // 我想知道这个使用assert_eq!的代码替换（展开）后会是什么样子！
    // trace_macros!(true);
    assert_eq!(10 * 10 * 10 + 9 * 9 * 9, 12 * 12 * 12 + 1 * 1 * 1);
    // trace_macros!(false);

    type RoomId = String; // 每个房间都有唯一的名字
    type RoomExits = Vec<(char, RoomId)>; // ……并且存在一个出口列表
    type RoomMap = HashMap<RoomId, RoomExits>; // 房间名和出口的简单映射表

    // 创建一个简单映射表
    let mut map = RoomMap::new();
    map.insert(
        "Cobble Crawl".to_string(),
        vec![('W', "Debris Room".to_string())],
    );
    map.insert(
        "Debris Room".to_string(),
        vec![
            ('E', "Cobble Crawl".to_string()),
            ('W', "Sloping Canyon".to_string()),
        ],
    );

    serde_json::to_writer(&mut std::io::stdout(), &map).unwrap();
    println!("{:?}", "---");

    echo_main("127.0.0.1:18080").expect("error running echo server: ");
}

#[cfg(test)]
mod tests {
    #[test]
    #[allow(unconditional_panic, unused_must_use)]
    #[should_panic(expected = "divide by zero")]
    fn test_divide_by_zero_error() {
        1 / 0; // 应该panic！
    }
}
