#[test]
fn type__() {
    // 前缀 checked_、wrapping_、saturating_ 或 overflowing_

    // checked_ 运算会返回结果的 Option 值：如果数学意义上正确的结果可以表示为该类型的值，那么就为 Some(v)，否则为 None。
    assert_eq!(10_u8.checked_add(20), Some(30)); // 加法add
    assert_eq!(100_u8.checked_add(200), None);
    10_u8.checked_sub(11) == None; // 减法sub
    (-128_i8).checked_neg() == None; // 取负neg
    3_u8.checked_pow(4) == Some(81); // 求幂pow
    assert_eq!((-128_i8).checked_div(-1), None); //  带符号的 n 位类型可以表示 -2n-1，但不足以表示 2n-1

    // wrapping_ 运算会返回与“数学意义上正确的结果”对“值类型范围”取模的值相等的值。
    64_u16.wrapping_div(8) == 8; // 除法div
    (-32768_i16).wrapping_rem(-1) == 0; // 求余rem
    (-32768_i16).wrapping_abs() == -32768; // 绝对值abs
    10_u32.wrapping_shl(34) == 40; // 按位左移shl
    40_u64.wrapping_shr(66) == 10; // 按位右移shr
                                   //  第一个结果可以表示为 u16，第二个则不能，所以会得到 250000  对 2**16的模
    assert_eq!(100_u16.wrapping_mul(200), 20000);
    assert_eq!(500_u16.wrapping_mul(500), 53392); // 250000 溢出，会取 余数     250000 % 2**16
                                                  //  对有符号类型的运算可能会回绕为负值
    assert_eq!(500_i16.wrapping_mul(500), -12144);
    //  在移位运算中，移位距离会在值的大小范围内回绕，
    //  所以在 16 位类型中移动 17 位就相当于移动了 1 位
    assert_eq!(5_i16.wrapping_shl(17), 10);

    // saturating_ 运算会返回最接近“数学意义上正确结果”的可表达值。换句话说，结果“紧贴着”该类型可表达的最大值和最小值。
    assert_eq!(32760_i16.saturating_add(10), 32767); //
    assert_eq!((-32760_i16).saturating_sub(10), -32768);
    128_u8.saturating_mul(3) == 255; // 乘法mul
    println!("{:?}", (-32760_i16).saturating_sub(10));

    // overflowing_ 运算会返回一个元组 (result, overflowed)，其中 result 是函数的回绕版本所返回的内容，而 overflowed 是一个布尔值，指示是否发生过溢出。
    assert_eq!(255_u8.overflowing_sub(2), (253, false));
    assert_eq!(255_u8.overflowing_add(2), (1, true));
    // 移动17位对`u16`来说太大了
    assert_eq!(5_u16.overflowing_shl(17), (10, true)); // (5 << 17) % 65535

    assert!((-1. / f32::INFINITY).is_sign_negative());
    assert_eq!(-f32::MIN, f32::MAX);
    assert_eq!(5f32.sqrt() * 5f32.sqrt(), 5.); // 按IEEE的规定，它精确等于5.0
    assert_eq!((-1.01f64).floor(), -2.0);
    println!("{}", (2.0_f64).sqrt());
    println!("{}", f64::sqrt(2.0));

    assert_eq!(false as i32, 0);
    assert_eq!(true as i32, 1);
    assert_eq!(1 != 0, true);

    assert_eq!('*'.is_alphabetic(), false);
    assert_eq!('β'.is_alphabetic(), true);
    assert_eq!('8'.to_digit(10), Some(8));
    assert_eq!('ಠ'.len_utf8(), 3);
    assert_eq!(std::char::from_digit(2, 10), Some('2'));

    let text = "I see the eigenvalue in thine eye";
    let (head, tail) = text.split_at(21);
    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");
    let text = "I see the eigenvalue in thine eye";
    let temp = text.split_at(21);
    let head = temp.0;
    let tail = temp.1;
    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");

    let mut chaos = [3, 5, 4, 1, 2];
    chaos.sort();
    assert_eq!(chaos, [1, 2, 3, 4, 5]);

    let t = (12, "eggs");
    let b = Box::new(t); // 在堆中分配一个元组

    let v: Vec<i32> = (0..5).collect();
    assert_eq!(v, [0, 1, 2, 3, 4]);

    let mut v = Vec::with_capacity(2);
    assert_eq!(v.len(), 0);
    assert_eq!(v.capacity(), 2);

    v.push(1);
    v.push(2);
    assert_eq!(v.len(), 2);
    assert_eq!(v.capacity(), 2);

    v.push(3);
    assert_eq!(v.len(), 3);
    // 通常会打印出"capacity is now 4":
    println!("capacity is now {}", v.capacity());

    let mut v = vec![10, 20, 30, 40, 50];

    // 在索引为3的元素处插入35
    v.insert(3, 35);
    assert_eq!(v, [10, 20, 30, 35, 40, 50]);

    // 移除索引为1的元素
    v.remove(1);
    assert_eq!(v, [10, 30, 35, 40, 50]);

    // println!("{}",(-4).abs());//  can't call method `abs` on ambiguous numeric type `{integer}`
    println!("{}", (-4_i32).abs());

    // 将命令行参数作为字符串的向量
    let languages: Vec<String> = std::env::args().skip(1).collect();
    for l in languages {
        println!(
            "{}: {}",
            l,
            if l.len() % 2 == 0 {
                "functional"
            } else {
                "imperative"
            }
        );
    }
}
