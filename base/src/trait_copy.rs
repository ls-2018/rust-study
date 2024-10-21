#[derive(Clone, Debug)]

struct Atype {
    num: u32,
    a_vec: Vec<u32>, // 动态数组资源在堆内存中
}

#[derive(Clone)]

struct Point {
    x: u32,
    y: u32,
}

#[derive(Copy, Clone)]

struct Point2 {
    x: u32,
    y: u32,
}

#[test]

fn main() {
    let a = Atype {
        num: 100,
        a_vec: vec![10, 20, 30],
    };

    let mut b = a.clone(); // 克隆，也将堆内存中的Vec资源部分克隆了一份
    b.num = 200; // 更改b的值
    b.a_vec[0] = 11;

    b.a_vec[1] = 21;

    b.a_vec[2] = 31;

    println!("{a:?}"); // 对比两份值
    println!("{b:?}");

    let a = Point { x: 10, y: 10 };

    let b = a; // 这里发生了所有权move，a在后续不能使用了
               // let c = a; // 这里发生了所有权move，a在后续不能使用了
    let a = Point2 { x: 10, y: 10 };

    let b = a; // 这里发生了所有权move，a在后续不能使用了
    let c = a; // 这里发生了所有权move，a在后续不能使用了

    let a: &str = "123456";

    let s: String = a.to_owned(); // 将引用转换为所有权实例
    println!("{}", a);

    println!("{}", s);
}
