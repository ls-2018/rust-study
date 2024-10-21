// 获取了上下文环境变量的所有权，对应 FnOnce。
// 只获取了上下文环境变量的 &mut 引用，对应 FnMut。
// 只获取了上下文环境变量的 & 引用，对应 Fn。

#[test]

fn main_fn_once() {
    // 获取了上下文环境变量的所有权，对应 FnOnce。

    let range = 0..10;

    let get_range_count = || range.count();

    assert_eq!(get_range_count(), 10); // ✅
                                       // get_range_count(); // ❌
}

#[test]

fn main_fn_mut() {
    // 代表的闭包类型只能被调用一次 只获取了上下文环境变量的 &mut 引用，对应 FnMut。
    // 代表的闭包类型可以被调用多次，并且能修改上下文环境变量的值，
    // 不过有一些副作用，在某些情况下可能会导致错误或者不可预测的行为。
    let nums = vec![0, 4, 2, 8, 10, 7, 15, 18, 13];

    let mut min = i32::MIN;

    let ascending = nums
        .into_iter()
        .filter(|&n| {
            if n <= min {
                false
            } else {
                min = n; // 这里修改了环境变量min的值
                true
            }
        })
        .collect::<Vec<_>>();
    assert_eq!(vec![0, 4, 8, 10, 15, 18], ascending); // ✅
}

#[test]

fn main_fn() {
    // 只获取了上下文环境变量的 & 引用，对应 Fn。
    // 代表的闭包类型可以被调用多次，但是对上下文环境变量没有副作用

    let nums = vec![0, 4, 2, 8, 10, 7, 15, 18, 13];

    let min = 9;

    let greater_than_9 = nums.into_iter().filter(|&n| n > min).collect::<Vec<_>>();

    assert_eq!(vec![10, 15, 18, 13], greater_than_9); // ✅

    dot_product::<3>([0.2, 0.4, 0.6], [0., 0., 1.]);
    dot_product([1f64, 4f64], [1f64, 2f64]);
}

fn dot_product<const N: usize>(a: [f64; N], b: [f64; N]) -> f64 {
    let mut sum = 0.;
    for i in 0..N {
        sum += a[i] * b[i];
    }
    sum
}

//1、 一个不修改变量的非 move 闭包只持有共享引用，这些引用既能 Clone 也能 Copy，所以闭包也能 Clone 和 Copy
//2、另外，一个会修改值的非 move 闭包在其内部表示中也可以有可变引用。可变引用既不能 Clone，也不能 Copy
//3、如果 move 闭包捕获的所有内容都能 Copy，那它就能 Copy。如果 move 闭包捕获的所有内容都能 Clone，那它就能 Clone。

// 不拥有任何资源的简单类型可以是 Copy 类型，对这些简单类型赋值会创建源的副本，而不会移动值并使源回到未初始化状态。
