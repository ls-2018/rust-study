use once_cell::sync::OnceCell;
use std::collections::HashMap;

#[derive(Debug)]
pub struct AppState(HashMap<String, String>);

static APP_GLOBAL_STATE: OnceCell<AppState> = OnceCell::new();

fn main() {
    // Initialize the global state
    APP_GLOBAL_STATE.get_or_init(new);
    let v = APP_GLOBAL_STATE.get().unwrap();
    println!("{:?}", v);
}
fn new() -> AppState {
    let mut map = HashMap::new();
    map.insert("hello".to_string(), "world".to_string());
    AppState(map)
}
