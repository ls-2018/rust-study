#[test]
fn xx() {
    let mut also_spaceless = "con".to_string();
    also_spaceless.extend("tri but ion".split_whitespace());
    assert_eq!(also_spaceless, "contribution");
    // let mut letter = String::new();
    // writeln!(letter, "Whose {} these are I think I know", "rutabagas").expect("TODO: panic message");
    // writeln!(letter, "His house is in the village though;").expect("TODO: panic message");
    // assert_eq!(letter, "Whose rutabagas these are I think I know\n\
    //                 His house is in the village though;\n");
}

use std::fmt;
use std::io::{Result, Write};

#[test]
fn main() -> Result<()> {
    let mut w = Vec::new();
    writeln!(&mut w)?;
    writeln!(&mut w, "test")?;
    writeln!(&mut w, "formatted {}", "arguments")?;

    assert_eq!(&w[..], "\ntest\nformatted arguments\n".as_bytes());

    let parenthetical = "(".to_string() + ")";

    assert_eq!(format!("{}, wow", "doge"), "doge, wow");
    assert_eq!(format!("{}", true), "true");
    assert_eq!(
        format!("({:.3}, {:.3})", 0.5, f64::sqrt(3.0) / 2.0),
        "(0.500, 0.866)"
    );

    use std::borrow::Cow;

    let x = std::env::var("USER")
        .map(|v| Cow::Owned(v))
        .unwrap_or(Cow::Borrowed("whoever you are"));

    let x = "a".to_string();
    // .into(); // 对应类型 实现了 impl From<(i32, i32)> for Point

    let x = std::env::var("USER")
        .map(|v| v.into())
        .unwrap_or(Cow::Borrowed("whoever you are"));

    assert_eq!(format!("{}", "bookends"), "bookends");
    assert_eq!(format!("{:4}", "bookends"), "bookends");
    assert_eq!(format!("{:12}", "bookends"), "bookends    ");
    assert_eq!(format!("{:.4}", "bookends"), "book");
    assert_eq!(format!("{:.12}", "bookends"), "bookends");
    assert_eq!(format!("{:12.20}", "bookends"), "bookends    ");
    assert_eq!(format!("{:4.20}", "bookends"), "bookends");
    assert_eq!(format!("{:4.6}", "bookends"), "booken");
    assert_eq!(format!("{:6.4}", "bookends"), "book  ");
    assert_eq!(format!("{:<12}", "bookends"), "bookends    ");
    assert_eq!(format!("{:^12}", "bookends"), "  bookends  ");
    assert_eq!(format!("{:>12}", "bookends"), "    bookends");
    assert_eq!(format!("{:=^12}", "bookends"), "==bookends==");
    assert_eq!(format!("{:*>12.4}", "bookends"), "********book");

    assert_eq!(format!("{}", 1234), "1234");
    assert_eq!(format!("{:+}", 1234), "+1234");
    assert_eq!(format!("{:12}", 1234), "        1234");
    assert_eq!(format!("{:2}", 1234), "1234");
    assert_eq!(format!("{:+12}", 1234), "       +1234");
    assert_eq!(format!("{:012}", 1234), "000000001234");
    assert_eq!(format!("{:+012}", 1234), "+00000001234");
    assert_eq!(format!("{:<12}", 1234), "1234        ");
    assert_eq!(format!("{:^12}", 1234), "    1234    ");
    assert_eq!(format!("{:>12}", 1234), "        1234");
    assert_eq!(format!("{:>1$}", 1234, 10), "      1234"); // 在运行期选择字段宽度

    println!("{:?}", format!("{:>width$}", "content", width = 10));
    println!(
        "{:?}",
        format!("{:>width$.limit$}", 10.12345, width = 5, limit = 3)
    );
    println!("{:?}", format!("{:.*}", 3, 1.2345));

    assert_eq!(format!("{:<+12}", 1234), "+1234       ");
    assert_eq!(format!("{:^+12}", 1234), "   +1234    ");
    assert_eq!(format!("{:>+12}", 1234), "       +1234");
    assert_eq!(format!("{:r^12}", 1234), "rrrr1234rrrr");
    assert_eq!(format!("{:b}", 1234), "10011010010");
    assert_eq!(format!("{:12o}", 1234), "        2322");
    assert_eq!(format!("{:+12x}", 1234), "        +4d2");
    assert_eq!(format!("{:+12X}", 1234), "        +4D2");
    assert_eq!(format!("{:+#12x}", 1234), "      +0x4d2");
    assert_eq!(format!("{:+#012x}", 1234), "+0x0000004d2");
    assert_eq!(format!("{:+#06x}", 1234), "+0x4d2");
    assert_eq!(format!("{:#?}", 1234), "1234"); // 调试视图
    assert_eq!(format!("{}", 1234.5678), "1234.5678");
    assert_eq!(format!("{:.2}", 1234.5678), "1234.57");
    assert_eq!(format!("{:.6}", 1234.5678), "1234.567800");
    assert_eq!(format!("{:12}", 1234.5678), "   1234.5678");
    assert_eq!(format!("{:12.2}", 1234.5678), "     1234.57");
    assert_eq!(format!("{:12.6}", 1234.5678), " 1234.567800");
    assert_eq!(format!("{:012.6}", 1234.5678), "01234.567800");
    assert_eq!(format!("{:e}", 1234.5678), "1.2345678e3");
    assert_eq!(format!("{:.3e}", 1234.5678), "1.235e3");
    assert_eq!(format!("{:12.3e}", 1234.5678), "     1.235e3");
    assert_eq!(format!("{:12.3E}", 1234.5678), "     1.235E3");

    use std::rc::Rc;
    let original = Rc::new("mazurka".to_string());
    let cloned = original.clone();
    let impostor = Rc::new("mazurka".to_string());
    println!("text:     {}, {}, {}", original, cloned, impostor);
    println!("pointers: {:p}, {:p}, {:p}", original, cloned, impostor);

    Ok(())
}
