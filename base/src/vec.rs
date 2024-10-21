#![allow(unused)]
#![allow(unused_variables)]

#[test]
fn vec_test() {


    let s1 = String::from("aaa");

    let s2 = String::from("bbb");

    let s3 = String::from("ccc");

    let s4 = String::from("ddd");
    let v = vec![s1, s2, s3, s4];
    v[0].clone();

    let a = &v[0]; // 明确a只获得v中第一个元素的引用

    let v = vec![1, 2, 3, 4];

    let a = &v[0]; // 明确a只获得v中第一个元素的引用

    // 构建一个由字符串"101"、"102"……"105"组成的向量
    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }
    println!("{:?}", v);
    // 方法一：从向量的末尾弹出一个值：
    let fifth = v.pop().expect("vector empty!");
    assert_eq!(fifth, "105");
    println!("{:?}", v);
    // 方法二：将向量中指定索引处的值与最后一个值互换，并把前者移动出来：
    let second = v.swap_remove(1); // 最后一个为空值时，只会把这个位置的数据删除
    assert_eq!(second, "102");
    println!("{:?}", v);
    // 方法三：把要取出的值和另一个值互换：
    let third = std::mem::replace(&mut v[2], "substitute".to_string());
    assert_eq!(third, "103");
    println!("{:?}", v);
    // 看看向量中还剩下什么
    assert_eq!(v, vec!["101", "104", "substitute"]);
}
