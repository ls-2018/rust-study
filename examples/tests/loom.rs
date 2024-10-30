use loom::sync::atomic::AtomicUsize;
use loom::sync::Mutex;
use loom::thread;

use std::rc::Rc;
use std::sync::atomic::Ordering::SeqCst;

#[test]
fn mutex_enforces_mutal_exclusion() {
    loom::model(|| {
        let data = Rc::new((Mutex::new(0), AtomicUsize::new(0)));

        let ths: Vec<_> = (0..2)
            .map(|_| {
                let data = data.clone();

                thread::spawn(move || {
                    let mut locked = data.0.lock().unwrap();
                    let prev = data.1.fetch_add(1, SeqCst);
                    assert_eq!(prev, *locked);
                    *locked += 1;
                })
            })
            .collect();

        for th in ths {
            th.join().unwrap();
        }

        let locked = data.0.lock().unwrap();

        assert_eq!(*locked, data.1.load(SeqCst));
    });
}
