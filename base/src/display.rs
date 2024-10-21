use std::fmt;

#[derive(Default)]

struct Point {
    x: i32,
    y: i32,
}

// 为Point实现 Display
impl fmt::Display for Point {
    // 实现唯一的fmt方法，这里定义用户自定义的格式
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y) // write!宏向stdout写入
    }
}

#[test]

fn main() {
    println!("origin: {}", Point::default());

    // 打印出 "origin: (0, 0)"
    // 在 format! 中用 "{}" 将类型表示/转换为 String
    let stringified = format!("{}", Point::default());

    assert_eq!("(0, 0)", stringified); // ✅
}

#[test] // ✅
fn display_point() {
    let origin = Point::default();

    assert_eq!(format!("{}", origin), "(0, 0)");
}

#[test] // ✅
fn point_to_string() {
    let origin = Point::default();

    assert_eq!(origin.to_string(), "(0, 0)");
}

#[test] // ✅
fn display_equals_to_string() {
    let origin = Point::default();

    assert_eq!(format!("{}", origin), origin.to_string());
}
