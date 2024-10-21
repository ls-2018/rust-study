use std::path::Path;

fn x() {
    let p = "/home/jimb/.emacs";
    // p.as_ref() as &Path;
    let dot_emacs = std::fs::File::open(p);

    let a = "asdasd"; //  栈
    let c = a.to_string(); // 堆
    let b = &a;
}
