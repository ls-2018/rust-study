use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

#[test]
fn m() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        sleep(Duration::from_secs(4));
        println!("1");
        *started = true;
        cvar.notify_one();
    });

    let (lock, cvar) = &*pair;
    sleep(Duration::from_secs(2));
    let mut started = lock.lock().unwrap();
    while !*started {
        println!("2 {}", started);
        started = cvar.wait(started).unwrap();
    }
    println!("3 {}", started);
}
