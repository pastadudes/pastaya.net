use wasm_bindgen::prelude::*;
use web_sys::{Storage, window};

fn storage() -> Storage {
    window().unwrap().local_storage().unwrap().unwrap()
}

fn read_count() -> u32 {
    storage()
        .get_item("clickCounter")
        .ok()
        .flatten()
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(0)
}

fn write_count(v: u32) {
    let _ = storage().set_item("clickCounter", &v.to_string());
}

#[wasm_bindgen]
pub fn init_counter() -> u32 {
    read_count()
}

#[wasm_bindgen]
pub fn increment_counter() -> u32 {
    let next = read_count().saturating_add(1);
    write_count(next);
    next
}
