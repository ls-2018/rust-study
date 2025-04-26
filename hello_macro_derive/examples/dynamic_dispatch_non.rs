use enum_dispatch::enum_dispatch;

#[enum_dispatch]
enum BehaviorEnum {
    // 每一个常量都会 实现,Behavior 并且可以直接通过BehaviorEnum.trait_method,不用通过子类型
    ImplementorA(ImplementorA),
    ImplementorB(ImplementorB),
}

// impl Behavior for BehaviorEnum
#[enum_dispatch(BehaviorEnum)]
trait Behavior {
    fn trait_method(&self);
}

struct ImplementorA {}

impl Behavior for ImplementorA {
    fn trait_method(&self) {
        println!("ImplementorA::trait_method");
    }
}

struct ImplementorB {}
impl Behavior for ImplementorB {
    fn trait_method(&self) {
        println!("ImplementorB::trait_method");
    }
}

fn main() {
    let a: BehaviorEnum = ImplementorA {}.into();
    // let a: BehaviorEnum = ImplementorB{}.into();
    a.trait_method();
}
