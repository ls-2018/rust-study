use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let mut log_file = OpenOptions::new().append(true).create(true).open("/tmp/log-file-name").expect("failed to open log file");
    log_file.write_all("asdasds".as_ref()).unwrap()
}
