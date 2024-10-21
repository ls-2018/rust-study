use enum_dispatch::enum_dispatch;
#[enum_dispatch]
enum BehaviorEnum {
    ImplementorA(ImplementorA),
}

#[enum_dispatch(BehaviorEnum)]
trait Behavior {
    fn trait_method(&self);
}

pub struct ImplementorA {}

impl ImplementorA {
    fn new() -> Self {
        ImplementorA {}
    }
}

impl Behavior for ImplementorA {
    fn trait_method(&self) {
        println!("ImplementorA::trait_method");
    }
}

// impl TryInto<ImplementorA> for BehaviorEnum {
//     type Error = &'static str;
//     fn try_into(
//         self,
//     ) -> Result<
//         ImplementorA,
//         <Self as TryInto<ImplementorA>>::Error,
//     > {
//         match self {
//             BehaviorEnum::ImplementorA(v) => Ok(v),
//         }
//     }
// }
// impl Behavior for BehaviorEnum {
//     #[inline]
//     fn trait_method(&self) {
//         match self {
//             BehaviorEnum::ImplementorA(inner) => Behavior::trait_method(inner),
//         }
//     }
// }

fn main() {
    let a: BehaviorEnum = ImplementorA::new().into();
    a.trait_method(); // 飘红没问题
}
