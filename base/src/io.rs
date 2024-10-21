use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::Write;
use std::os::unix::fs::symlink;
use std::path::Path;
use std::process::*;

#[test]
fn asx() {
    // let mut child =
    //     Command::new("grep")
    //         .arg("-e")
    //         .arg("a.*e.*i.*o.*u")
    //         .stdin(Stdio::piped())
    //         .spawn().expect("");
    //
    // let mut to_child = child.stdin.take().unwrap();
    // to_child.write("go".as_ref());
    // for word in my_words {
    //     writeln!(to_child, "{}", word);
    // }
    // drop(to_child);  // 关闭grep的stdin，以便让它退出
    // child.wait();

    assert_eq!(
        Path::new("/home/fwolfe/program.txt").parent(),
        Some(Path::new("/home/fwolfe"))
    );

    use std::ffi::OsStr;
    assert_eq!(
        Path::new("/home/fwolfe/program.txt").file_name(),
        Some(OsStr::new("program.txt"))
    );
    let path1 = Path::new("/usr/share/dict");
    assert_eq!(path1.join("words"), Path::new("/usr/share/dict/words"));
    let abs_path = std::env::current_dir().expect("").join("a.txt");
    println!("{:?}", abs_path);
    let file = Path::new("/home/jimb/calendars/calendar-18x18.pdf");
    assert_eq!(
        file.ancestors().collect::<Vec<_>>(),
        vec![
            Path::new("/home/jimb/calendars/calendar-18x18.pdf"),
            Path::new("/home/jimb/calendars"),
            Path::new("/home/jimb"),
            Path::new("/home"),
            Path::new("/")
        ]
    );
    Path::new("").metadata().expect_err("NotFound");
}

/// 把现有目录`src`复制到目标路径`dst`
fn copy_dir_to(src: &Path, dst: &Path) -> io::Result<()> {
    if !dst.is_dir() {
        fs::create_dir(dst)?;
    }
    for entry_result in src.read_dir()? {
        let entry = entry_result?;
        let file_type = entry.file_type()?;
        copy_to(&entry.path(), &file_type, &dst.join(entry.file_name()))?;
    }
    Ok(())
}

/// 把`src`中的任何内容复制到目标路径`dst`
fn copy_to(src: &Path, src_type: &fs::FileType, dst: &Path) -> io::Result<()> {
    if src_type.is_file() {
        fs::copy(src, dst)?;
    } else if src_type.is_dir() {
        copy_dir_to(src, dst)?;
    } else if src_type.is_symlink() {
        let target = src.read_link()?;
        symlink(target, dst)?;
    } else {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("don't know how to copy: {}", src.display()),
        ));
    }
    Ok(())
}
