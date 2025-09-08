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
