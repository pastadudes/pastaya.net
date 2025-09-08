/*
 * ;ib.rs - a replacement for a button counter
 * Copyright (C) 2025 pastaya
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

// HEY! special_effects() is a STUB!! PLEASE HELP EXPAND IT!
use wasm_bindgen::prelude::*;
use web_sys::{Storage, console, window};

// setup local storage instead of cookies

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
    special_effects();
    next
}

#[wasm_bindgen]
pub fn special_effects() {
    let count = read_count(); // debating if i should've made it a reference

    match count {
        21 => console::log_1(&"ay man whats 9 + 10?? 21. u stupid".into()),
        67 => console::log_1(&"HOW MANY LETTERS IN MANGO AND IN MUSTARD??? 6 7".into()),
        _ => {} // how am i supposed to do nothing???

                // TODO: make these ACTUALLY change DOM and play sound effects
    }
}
