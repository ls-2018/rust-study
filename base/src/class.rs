trait Operate {
    fn plus(self) -> Self;
}

// trait和类型至少有一个要在当前的模块中
impl Operate for i64 {
    fn plus(self) -> Self {
        self + self
    }
}

struct MyI8(i8);

impl MyI8 {
    fn foo(&self) {
        println!("foo :{}", self.0);
    }
}

#[test]

fn main() {
    let a: i64 = 1;

    println!("{}", a.plus());

    let a = MyI8(2i8);

    a.foo();

    println!("{:06x}", 16) // 这里表示打印出值的 16 进制形式，占位 6
                           // 个宽度，不足的用 0 补齐
}
