#[test]

//  Arc 可以安全地在线程之间直接共享，而普通 Rc 会使用更快的非线程安全代码来更新其引用计数。
fn rc() {
    use std::rc::Rc;
    // 拥有型，但不可变
    // Rust能推断出所有这些类型，这里写出它们只是为了讲解时清晰
    let s: Rc<String> = Rc::new("shirataki".to_string());
    // Rc 指针的引用目标通常都可以共享，因此就不能是可变的。
    let t: Rc<String> = s.clone();
    let u: Rc<String> = s.clone();
}

//  非拥有型指针
use std::collections::HashMap;
type Table = HashMap<String, Vec<String>>;

fn show(table: &Table) {
    // 传入了共享引用
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!("  {}", work);
        }
    }
}
fn sort_works(table: &mut Table) {
    for (_artist, works) in table {
        works.sort();
    }
}
#[test]
fn _ref() {
    let mut table = Table::new();
    table.insert(
        "Gesualdo".to_string(),
        vec![
            "many madrigals".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );
    table.insert(
        "Caravaggio".to_string(),
        vec![
            "The Musicians".to_string(),
            "The Calling of St. Matthew".to_string(),
        ],
    );
    table.insert(
        "Cellini".to_string(),
        vec![
            "Perseus with the head of Medusa".to_string(),
            "a salt cellar".to_string(),
        ],
    );
    show(&table);
    assert_eq!(table["Gesualdo"][0], "many madrigals");
    sort_works(&mut table);
    // 不可以继续使用 table

    let mut v = vec![1973, 1968];
    v.sort(); // 隐式借用对v的可变引用
    (&mut v).sort(); // 等效，但是更烦琐
    println!("{:?}", v);

    let x = 10;
    let y = 10;

    let rx = &x;
    let ry = &y;

    let rrx = &rx;
    let rry = &ry;

    assert!(rrx <= rry);
    assert_eq!(rx, ry); // 它们引用的目标值相等
    assert!(!std::ptr::eq(rx, ry)); // 但所占据的地址（自身的值）不同
}
