use std::ops::Add;
use std::ops::Mul;

// trait Add<Rhs = Self> {// Rhs 可以是任意名, 默认类型是 Self
//     type Output;
//     fn add(self, rhs: Rhs) -> Self::Output;
// }

//  Copy 和 Clone 的区别是，Copy 是浅拷贝只复制一层
// 一旦一个类型实现了 Copy，它就会具备一个特别重要的特性：
// 再赋值的时候会复制一份自身。那么就相当于新创建一份所有权(多份了)
//  Vec 只能有一个所有权

#[derive(Clone)]

struct Point {
    x: i32,
    y: i32,
}

// 为 Point 类型实现 Add trait，这样两个Point实例就可以直接相加
impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul for Point {
    type Output = Point;

    fn mul(self, rhs: Point) -> Point {
        Point {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

#[test]

fn main() {
    let p1 = Point { x: 1, y: 2 };

    let p2 = Point { x: 3, y: 4 };

    let p3 = p1.clone() + p2.clone(); // 这里直接用+号作用在两个Point实例上
                                      // let p3 = p1 + p2; // 这里直接用+号作用在两个Point实例上,会导致所有权转移
    assert_eq!(p3.x, p1.x + p2.x); // ✅
    assert_eq!(p3.y, p1.y + p2.y); // ✅

    // =默认是move
    let a = San { x: 10, y: 10 };

    let b = a; // 这里发生了复制，a在后续可以继续使用
               // let c = a; // 这里又复制了一份，这下有3份了
    println!("{}", a.y);

    println!("{}", b.y);

    let a: &str = "123456";

    let s: String = a.to_owned(); // 将引用转换为所有权实例

    println!("{}", a);

    println!("{}", a);

    println!("{}", s);
}

#[derive(Copy, Clone)]

struct San {
    x: u32,
    y: u32,
}

//
