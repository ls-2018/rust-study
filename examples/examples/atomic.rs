use lazy_static::lazy_static;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex, mpsc};

lazy_static! {
    // 使用 lazy_static! 会在每次访问静态数据时产生很小的性能成本
    // 通过 lazy_static! 宏定义的变量允许你使用任何喜欢的表达式进行初始化，该表达式会在第一次解引用变量时运行，并保存该值以供后续操作使用。
    static ref HOSTNAME: Mutex<String> = Mutex::new(String::new());
}

fn main() {
    // Rust 具有用于安全地共享变化的值的类型：Mutex、RwLock 和原子化类型。
    // 原子化全局变量仅限于简单的整数和布尔值。
    // 全局变量必须以某种方式成为线程安全的
    static PACKETS_SERVED: AtomicUsize = AtomicUsize::new(0);
    PACKETS_SERVED.fetch_add(1, Ordering::SeqCst);

    let cancel_flag = Arc::new(AtomicBool::new(false));
    cancel_flag.store(true, Ordering::SeqCst);
    let worker_cancel_flag = cancel_flag.clone();
    worker_cancel_flag.load(Ordering::SeqCst); // 内存排序
}
