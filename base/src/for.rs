#[test]

fn main() {
    let mut counter = 0;

    // 这里，接收从循环体中返回的值，对result进行初始化
    let result = loop {
        counter += 1;

        if counter == 10 {
            // 使用break跳出循环，同时带一个返回值回去
            break counter * 2;
        }
    };

    println!("The result is {result}");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    // 左闭右开区间
    for number in 1..4 {
        println!("{number}");
    }

    println!("--");

    // 左闭右闭区间
    for number in 1..=4 {
        println!("{number}");
    }

    println!("--");

    // 反向
    for number in (1..4).rev() {
        println!("{number}");
    }

    for ch in 'a'..='z' {
        println!("{ch}");
    }

    // garden::m(5, 'h');
    let a = 10u32; // 局部变量
                   // fn add_v1(x: u32) -> u32 { x + a } // 定义一个内部函数
    let add_v2 = |x: u32| -> u32 { x + a }; // 定义一个闭包
    let add_v2 = |x: u32| x + a; // 定义一个闭包
    let add_v2 = |x: u32| x + a; // 定义一个闭包
                                 // let result1 = add_v1(20);    // 调用函数
    let result2 = add_v2(20); // 调用闭包
    println!("{}", result2);
    use super::vegetables::echo;

    print!("{}", echo(1));

    let s = "I am a superman.".to_string();

    for i in 1..10 {
        let tmp_s = s.clone();

        println!("s is {}", tmp_s);
    }

    // 很重要
    // 一个所有权型变量的可变引用与不可变引用的作用域不能交叠
    // 同一个所有权型变量的可变借用之间的作用域也不能交叠
    let mut a = 10u32;

    let c = &a; // c的定义移到这里来了
    let b = &mut a;

    *b = 20;

    println!("{b}"); // 这里打印的变量换成b   // 默认会对所有权变量做不可变借用操作

    let s = String::from("中国你好");

    let _ = s.as_str().chars();

    let char_vec: Vec<char> = s.chars().collect();

    println!("{:?}", char_vec);

    for ch in s.chars() {
        println!("{:?}", ch);
    }
}
