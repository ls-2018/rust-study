use lazy_static::lazy_static;
use regex::Regex;
use std::io::BufRead;

lazy_static! {
    static ref SEMVER: Regex =
        Regex::new(r"(\d+)\.(\d+)\.(\d+)(-[-.[:alnum:]]*)?").expect("error parsing regex");
}

#[test]
fn x() {
    // 语义化版本号，比如0.2.1
    // 可以包含预发行版本后缀，比如0.2.1-alpha
    // （为简洁起见，没有“构建编号”元信息后缀）
    //
    // 注意，使用原始字符串语法r"..."是为了避免一大堆反斜杠
    let semver = Regex::new(r"(\d+)\.(\d+)\.(\d+)(-[-.[:alnum:]]*)?").expect("");

    // 简单搜索，返回布尔型结果
    let haystack = r#"regex = "0.2.5""#;
    assert!(semver.is_match(haystack));
    // 可以检索各个捕获组：
    let captures = semver
        .captures(haystack)
        .ok_or("semver regex should have matched")
        .expect("");
    assert_eq!(&captures[0], "0.2.5");
    assert_eq!(&captures[1], "0");
    assert_eq!(&captures[2], "2");
    assert_eq!(&captures[3], "5");

    assert_eq!(captures.get(4), None);
    assert_eq!(captures.get(3).unwrap().start(), 13);
    assert_eq!(captures.get(3).unwrap().end(), 14);
    assert_eq!(captures.get(3).unwrap().as_str(), "5");
    let haystack = "In the beginning, there was 1.0.0. \
                For a while, we used 1.0.1-beta, \
                but in the end, we settled on 1.2.4.";

    let matches: Vec<&str> = semver
        .find_iter(haystack)
        .map(|match_| match_.as_str())
        .collect();
    assert_eq!(matches, vec!["1.0.0", "1.0.1-beta", "1.2.4"]);

    // let stdin = std::io::stdin();
    // for line_result in stdin.lock().lines() {
    //     let line = line_result.expect("");
    //     if let Some(match_) = SEMVER.find(&line) {
    //         println!("{}", match_.as_str());
    //     }
    // }

    // 哈希器旨在累积求出一系列值的哈希值，因此仅哈希一个值有点儿大材小用
    use std::collections::hash_map::DefaultHasher;
    use std::hash::*;
    fn hash<T: ?Sized + Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }

    use unicode_normalization::UnicodeNormalization;

    // 不管左边的字符串使用哪种表示形式（无法仅仅通过观察得知），这些断言都成立
    assert_eq!("Phở".nfd().collect::<String>(), "Pho\u{31b}\u{309}");
    assert_eq!("Phở".nfc().collect::<String>(), "Phở");

    // 左侧使用了"ffi"连字符
    assert_eq!("① Difficulty".nfkc().collect::<String>(), "1 Difficulty");
}
