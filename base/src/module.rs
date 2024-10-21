mod module_a {

    pub trait Shape {
        fn play(&self) {
            println!("1");
        }
    }

    pub struct A;

    impl Shape for A {}
}

mod module_b {

    use super::module_a::Shape; // 引入这个trait use super::module_a::A;
    use super::module_a::A; // 这里只引入了另一个模块中的类型

    fn doit() {
        let a = A;

        a.play();
    }
}

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    // 第一次 impl
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    // 第二次 impl
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
