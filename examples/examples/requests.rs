use async_std::task;
use std::io::prelude::*;
use std::rc::Rc;
use std::{io, net};
// 原子化引用计数 Arc， 复制 Arc 智能指针而不是整个 GigabyteMap。这相当于增加一次引用计数
use lazy_static::lazy_static;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex, mpsc};
use std::thread::spawn;
// use crate::{fmt, std::io::ErrorKind::NotFound};
use std::io::{Error, Read};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt}; // 引入AsyncWriteExt trait

fn cheapo_request(host: &str, port: u16, path: &str) -> std::io::Result<String> {
    use std::net;
    let mut socket = net::TcpStream::connect((host, port))?;

    let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, host);
    socket.write_all(request.as_bytes())?;
    socket.shutdown(net::Shutdown::Write)?;

    let mut response = String::new();
    socket.read_to_string(&mut response)?;

    Ok(response)
}

async fn cheapo_request2(host: &str, port: u16, path: &str) -> std::io::Result<String> {
    use async_std::io::prelude::*;
    use async_std::net;

    let mut socket = net::TcpStream::connect((host, port)).await?;

    let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, host);
    socket.write_all(request.as_bytes()).await?;
    socket.shutdown(net::Shutdown::Write)?;

    let mut response = String::new();
    socket.read_to_string(&mut response).await?;

    Ok(response)
}

fn main() -> std::io::Result<()> {
    use async_std::task;

    let response = task::block_on(cheapo_request2("baidu.com", 80, "/"))?;
    println!("{:#?}", response);

    let requests = vec![("baidu.com".to_string(), 80, "/".to_string()), ("zhihu.com".to_string(), 80, "/".to_string())];

    let results = async_std::task::block_on(many_requests(requests));
    for result in results {
        match result {
            Ok(response) => println!("{}", response),
            Err(err) => eprintln!("error: {}", err),
        }
    }

    let input = async_std::io::stdin();
    let future = async {
        let mut line = String::new();

        // 这会返回`std::io::Result<usize>`
        input.read_line(&mut line).await?;

        println!("Read line: {}", line);

        Ok::<(), std::io::Error>(())
    };

    Ok(())
}

pub async fn many_requests(requests: Vec<(String, u16, String)>) -> Vec<std::io::Result<String>> {
    async fn cheapo_owning_request(host: String, port: u16, path: String) -> std::io::Result<String> {
        cheapo_request2(&host, port, &path).await
    }

    let mut handles = vec![];
    for (host, port, path) in requests {
        handles.push(
            task::spawn(async move { cheapo_request2(&host, port, &path).await }),
            // task::spawn_local(cheapo_owning_request(host, port, path)) // spawn_local 只会接受生命周期为 'static 的 Future
        ); // unstable
    }
    let mut results = vec![];
    for handle in handles {
        results.push(handle.await);
    }
    results
}

async fn reluctant() -> String {
    let res = {
        let string = Rc::new("ref-counted string".to_string());
        format!("Your splendid string: {}", string)
    };

    let requests = vec![("baidu.com".to_string(), 80, "/".to_string())];
    many_requests(requests).await;
    //  1、限制非send值的作用域，使其不跨越任何await表达式的作用域
    res
}

#[test]
fn mai2n() {
    use async_std::task;
    task::spawn(reluctant());
}

async fn doit() -> std::io::Result<()> {
    let mut file = File::create("._foo.txt").await?; // 创建文件
    file.write_all(b"hello, world!").await?; // 写入内容

    let mut file = File::open("._foo.txt").await?; // 打开文件
    let mut contents = vec![]; // 将文件内容读到contents动态数组里面，注意传入的是可变引用
    file.read_to_end(&mut contents).await?;
    println!("len = {}", contents.len());

    Ok(())
}

// #[tokio::main]// 多线程
// #[tokio::main(flavor = "current_thread")]  // 单线程
async fn main6() {
    async {}.await;
    let result = doit().await; // 注意这里的.await

    // 在这里执行异步任务
    let task_a = task::spawn(async { "hello world!" });
    // ...
    // 等待子任务结束，返回结果
    let result = task_a.await;
    assert_eq!(result, "hello world!");

    // 此任务跑在一个单独的线程中, 可以跑计算密集型任务
    let blocking_task = tokio::task::spawn_blocking(|| {
        // 在这里面可以执行阻塞线程的代码
    }); // 像下面这样用同样的方式等待这种阻塞式任务的完成
    blocking_task.await.unwrap();
}
// 等价于

fn main23() {
    // tokio::runtime::Builder::new_multi_thread()
    tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
        // 注意这里block_on，里面是异步代码
        println!("Hello world");
    })
}

struct GigabyteMap {}

fn process_files_in_parallel(glossary: Arc<GigabyteMap>) -> io::Result<()> {
    let mut thread_handles = Vec::new();
    for worklist in 1..=8 {
        // 对.clone()的调用只会克隆Arc并增加引用计数，并不会克隆GigabyteMap
        let glossary_for_child = glossary.clone();
        thread_handles.push(spawn(move || process_files(worklist, &glossary_for_child)));
    }
    for p in thread_handles {
        p.join().unwrap();
    }

    Ok(())
}

#[allow(unused_variables)]
fn process_files(p0: i32, p1: &Arc<GigabyteMap>) {}

fn mai1n() {
    let g = GigabyteMap {};
    process_files_in_parallel(Arc::new(g)).unwrap();
    let _ = mpsc::sync_channel::<i32>(10000);
    let (sender, receiver) = mpsc::channel(); // 线程通道
    let _ = sender.send("aa").is_err();
    println!("{:?}", receiver.recv());
    println!("Hello, world!");

    let c = Condvar::new();
    c.notify_one();
    c.notify_all();
}
