// 两种 胖指针，即携带某个值地址的双字值，以及要正确使用该值所需的某些额外信息  (切片、向量)
// 引用看起来很像 C 或 C++ 中的普通指针。但普通指针是不安全的，Rust 又如何保持对引用的全面控制呢？

#[test]
fn xxx() {
    // {
    //     let r;
    //     {
    //         let x = 1;
    //         r = &x;
    //     }
    //     assert_eq!(*r, 1);  // 错误：试图读取`x`所占用的内存
    // }

    f(&11);
    static WORTH_POINTING_AT: i32 = 1000;
    f(&WORTH_POINTING_AT);
}

static mut STASH: &i32 = &128;
// fn f(p: &i32) { // 仍然不够理想
//     unsafe {
//         STASH = p;
//     }
// }
fn f(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}

// fn f2<'a>(p: &'a i32) {}
//  这里，生命周期 'a（读作“tick A”）是 f2 的生命周期参数。
// <'a> 的意思是“对于任意生命周期 'a”，因此当我们编写 fn f<'a>(p: &'a i32) 时，
// 就定义了一个函数，该函数能接受对具有任意生命周期 'a 的 i32 型引用。

struct S {
    r: &'static i32, // r 只能引用贯穿程序整个生命周期的 i32 值
}

#[derive(Debug)]
struct S1<'a> {
    r: &'a i32, // 必须指定  声明周期
}

struct S2 {
    // r: &i32, // r 可能引用一个短声明周期的参数，
}

mod tests {

    struct S3<'a, 'b> {
        x: &'a i32,
        y: &'b i32,
    }

    #[test]
    fn sm() {
        let x = 10;
        let r;
        {
            let y = 20;
            {
                let s = S3 { x: &x, y: &y };
                //  让每个引用都有各自的生命周期，就可以解决所有问题：
                r = s.x;
            }
        }
        println!("{}", r);
        println!("{:?}", 5 * (41 - 32) / 9)
    }
}
