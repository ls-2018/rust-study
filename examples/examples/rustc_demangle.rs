use rustc_demangle::demangle;

fn main() {
    assert_eq!(demangle("_ZN4testE").to_string(), "test");
    assert_eq!(demangle("_ZN3foo3barE").to_string(), "foo::bar");
    assert_eq!(demangle("foo").to_string(), "foo");
    // With hash
    assert_eq!(format!("{}", demangle("_ZN3foo17h05af221e174051e9E")), "foo::h05af221e174051e9");
    // Without hash
    assert_eq!(format!("{:#}", demangle("_ZN3foo17h05af221e174051e9E")), "foo");
}
