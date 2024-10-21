// 只有满足对象安全（object safety）的 trait 才能被用作 trait object。
trait ObjectSafe {
    fn foo(&self) {}

    fn foo_mut(&mut self) {}

    fn foo_box(self: Box<Self>) {}
}

trait NotObjectSafe {
    const CONST: i32 = 1; // 不能包含关联常量

    fn foo() {} // 不能包含这样的关联函数
    fn selfin(self); // 不能将Self所有权传入 ,,,  但是我发现这个是可以的
    fn returns(&self) -> Self; // 不能返回Self
    fn typed<T>(&self, x: T) {} // 方法中不能有类型参数
}

//  不要在 trait 里面定义构造函数，比如 new 这种返回 Self 的关联函数。
// 你可以发现，确实在整个 Rust 生态中都没有将构造函数定义在 trait 中的习惯。
// trait 里面尽量定义传引用 &self 或 &mut self 的方法，而不要定义传值 self
// 的方法。

// 不要在 trait 里面定义构造函数，比如 new 这种返回 Self
// 的关联函数。你可以发现，确实在整个 Rust 生态中都没有将构造函数定义在 trait
// 中的习惯。 trait 里面尽量定义传引用 &self 或 &mut self 的方法，而不要定义传值
// self 的方法。
#[derive(Debug)]

struct Atype;

#[derive(Debug)]

struct Btype;

#[derive(Debug)]

struct Ctype;

trait TraitA {}

impl TraitA for Atype {}

impl TraitA for Btype {}

impl TraitA for Ctype {}

#[test]

fn foo() {
    let a = Atype;

    let b = Btype;

    let c = Ctype;

    let v: Vec<&dyn TraitA> = vec![&a, &b, &c];
}

fn doit(i: u32) -> Box<dyn TraitA> {
    // impl TraitA 作为函数返回值这种语法，其实也只是指代某一种类型而已

    // impl trait
    // 来说，它目前只能用于少数几个地方。一个是函数参数，另一个是函数返回值
    // trait 本身是一种非固定尺寸类型
    if i == 0 {
        let a = Atype;

        Box::new(a) // 在这个分支中返回类型a
    } else if i == 1 {
        let b = Btype;

        Box::new(b) // 在这个分支中返回类型b
    } else {
        let c = Ctype;

        Box::new(c) // 在这个分支中返回类型c
    }
}

// Box 的作用是可以保证获得里面值的所有权，必要的时候会进行内存的复制，
// 比如把栈上的值复制到堆中去。 一旦值到了堆中，就很容易掌握到它的所有权。
fn doit1111(x: impl TraitA) {}

fn doit11113<T: TraitA>(x: T) {}

// impl trait 用的是编译器静态展开，也就是编译时具化（单态化）。
fn doit11114(x: &dyn TraitA) {}

// 而 dyn trait 的版本不会在编译期间做任何展开，dyn TraitA
// 自己就是一个类型，这个类型相当于一个代理类型，
// 用于在运行时代理相关类型及调用对应方法。
// 既然是代理，也就是调用方法的时候需要多跳转一次，从性能上来说，
// 当然要比在编译期直接展开一步到位调用对应函数要慢一点。

fn xx() {
    use std::io::Write;
    let mut buf: Vec<u8> = vec![];
    // let writer: dyn Write = buf;  // 错误：`Write`的大小不是常量
    let mut buf: Vec<u8> = vec![];
    let writer: &mut dyn Write = &mut buf; // 正确
}
