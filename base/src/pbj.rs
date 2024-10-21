#[derive(Debug, Default)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 就像这样去实现 // area就是方法，被放在impl实现体中
    fn area(self) -> u32 {
        self.width * self.height
    }

    // 传入实例的所有权
    fn area2(&self) -> u32 {
        self.width * self.height
    }

    // 传入实例的不可变引用
    fn area3(&mut self) -> u32 {
        self.width * self.height
    }

    // 传入实例的可变引用

    fn numbers(rows: u32, cols: u32) -> u32 {
        rows * cols
    }

    // 关联函数，第一个参数不是self

    pub fn new(width: u32, height: u32) -> Self {
        println!("new");

        Rectangle { width, height }
    } // 构造函数
}

#[test]

fn o() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let x = &rect1;

    // println!("The area of the rectangle is {} square pixels.", rect1.area()); //
    // 传入rect
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area2()
    ); // 传入&rect1
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area2()
    ); // 传入&rect1
    println!("The area of the rectangle is {} square pixels.", x.area2()); // 传入&rect1
    println!("The area of the rectangle is {} square pixels.", x.area2()); // 传入&rect1
                                                                           // println!("The area of the rectangle is {} square pixels.", rect1.area3()); //
                                                                           // 传入&mut rect1

    println!("{}", Rectangle::numbers(1, 2));

    let rect1 = Rectangle::new(30, 50);

    let rect1: Rectangle = Default::default(); // 使用方式1
    let rect2 = Rectangle::default(); // 使用方式2
    println!("{:?}", rect1);

    println!("{:?}", rect2);

    // 实例化枚举
    let add = MyEnum::Add;

    // 实例化后执行枚举的方法
    add.run(100, 200);
}

enum Shape {
    Rectangle(Rectangle),
    Triangle((u32, u32), (u32, u32), (u32, u32)),
    Circle { origin: (u32, u32), radius: u32 },
}

// 给枚举变体一个起始数字值
enum Number {
    Zero = 0,
    One,
    Two,
}

// 给枚举每个变体赋予不同的值

enum MyEnum {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl MyEnum {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            // match 语句
            Self::Add => x + y,
            Self::Subtract => x - y,
            _ => 0,
        }
    }
}

#[test]

fn main() {
    let number = 13; // 你可以试着修改上面的数字值，看看下面走哪个分支
    println!("Tell me about {}", number);

    match number {
        // 匹配单个数字
        1 => println!("One!"),                             // 匹配几个数字
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"), // 匹配一个范围，左闭右闭区间
        13..=19 => println!("A teen"),                     // 处理剩下的情况
        _ => println!("Ain't special"),
    }

    let shape_a = MyEnum::Add;

    if let MyEnum::Add = shape_a {
        // 说 == 号不能作用在类型 MyEnum 上。 MyEnum 还不支持partialeq
        println!("1");
    } else {
        println!("10");
    }

    let MyEnum::Add = shape_a else {
        panic!("asdas")
        // MyEnum::Multiply
    };

    let a = (1, 2, 'a');

    let (b, c, d) = a;

    let mut shape_a = MyEnum::Add;

    let mut i = 0;

    while let MyEnum::Add = shape_a {
        // 注意这一句
        if i > 9 {
            println!("Greater than 9, quit!");

            shape_a = MyEnum::Multiply;
        } else {
            println!("`i` is `{:?}`. Try again.", i);

            i += 1;
        }
    }
}
