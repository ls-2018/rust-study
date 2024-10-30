// 定义一个 Trait，它是一个对象安全 trait，因为它只包含关联函数（没有泛型参数）。
trait Drawable {
    fn draw(&self);
}

// 实现 Drawable Trait 对于 Circle 结构体。
struct Circle;
impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle.");
    }
}

// 实现 Drawable Trait 对于 Square 结构体。
struct Square;
impl Drawable for Square {
    fn draw(&self) {
        println!("Drawing a square.");
    }
}

// 一个函数，它接受一个实现了 Drawable 的类型的对象。
fn draw_shape(shape: &dyn Drawable) {
    shape.draw();
}

fn main() {
    let circle = Circle;
    let square = Square;

    // 使用 Trait 对象来存储不同的类型。
    let shapes: Vec<&dyn Drawable> = vec![&circle, &square];

    // 遍历 shapes 并调用 draw 方法，这将在运行时解析。
    for shape in shapes.iter() {
        draw_shape(*shape);
    }
}
