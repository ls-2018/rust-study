use enum_dispatch::enum_dispatch;

#[enum_dispatch]
enum MyBehaviorEnum {
    MyImplementorA(MyImplementorA),
}

pub struct MyImplementorA {
    pub is_initialized: bool,
}

impl MyImplementorA {
    fn new() -> Self {
        MyImplementorA { is_initialized: false }
    }
}

#[enum_dispatch(MyBehaviorEnum)]
trait MyBehavior {
    fn my_trait_method(&self);
}

impl MyBehavior for MyImplementorA {
    fn my_trait_method(&self) {
        println!("MyImplementorA::my_trait_method::is_initialized:{}", self.is_initialized);
    }
}

// 等价于加上下边的
// impl MyBehavior for MyBehaviorEnum {
//     #[inline]
//     fn my_trait_method(&self) {
//         match self {
//             MyBehaviorEnum::MyImplementorA(inner) => MyBehavior::my_trait_method(inner),
//         }
//     }
// }
// impl TryInto<MyImplementorA> for MyBehaviorEnum {
//     type Error = &'static str;
//     fn try_into(self) -> Result<MyImplementorA, <Self as TryInto<MyImplementorA>>::Error, > {
//         match self {
//             MyBehaviorEnum::MyImplementorA(inner) => Ok(inner),
//         }
//     }
// }

fn main() {
    let a: MyBehaviorEnum = MyImplementorA::new().into();
    a.my_trait_method(); //no dynamic dispatch
}
