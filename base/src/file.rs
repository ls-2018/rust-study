#![recursion_limit = "256"]
#![feature(trace_macros)]
use actix_web::web::Path;
use serde_json::Value::String;
use std::fs::OpenOptions;
use std::io::Write;

fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}

pub fn write_log_entry(entry: std::fmt::Arguments) {
    // 尽量保持简单，所以每次只是打开文件
    let mut log_file = OpenOptions::new().append(true).create(true).open("/tmp/log-file-name").expect("failed to open log file");

    log_file.write_fmt(entry).expect("failed to write to log");
}

// `ident`, `block`, `stmt`, `expr`, `pat`, `ty`, `lifetime`
// `literal`, `path`, `meta`, `tt`, `item` and `vis`
#[macro_export]
macro_rules! log { // 在宏定义中的宏名后不需要叹号（!）
    ($format:tt, $($arg:expr),*) => (
        write_log_entry(format_args!($format, $($arg),*))
    )
}
#[macro_export]
macro_rules! bad_assert_eq {
    ($left:expr, $right:expr $(,)?) => {{
        match ($left, $right) {
            (left_val, right_val) => {
                println!("{}", left_val);
                println!("{}", right_val);
            }
        }
    }};
}
#[test]
fn m() {
    log!("{:?}\n", "asd".to_string());
    bad_assert_eq!("a", "b",);
}
#[test]
fn f() {
    let x = "asdasd";
    let x = "asdasd".to_string();

    use std::fs::File;
    let mut local_file = File::create("/tmp/hello.txt").expect("");
    say_hello(&mut local_file).expect("TODO: panic message"); // 正常
    let mut bytes = vec![];
    say_hello(&mut bytes).expect("TODO: panic message"); // 同样正常
    assert_eq!(bytes, b"hello world\n");
    log!("{:?}\n", "asd".to_string());
}
