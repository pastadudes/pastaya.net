/*
 * lib.rs - a replacement for a button counter
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

// my goal is for this to ACTUALLY be a good clicker game
use std::cell::Cell;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlImageElement, window};

// ---------- Clippy-clean thread-local counter ----------
#[warn(clippy::missing_const_for_thread_local)]
thread_local! {
    static COUNTER: Cell<u32> = Cell::new(0);
}

// we use local storage instead of cookies since cookies have a shitton of limits
fn storage() -> web_sys::Storage {
    window().unwrap().local_storage().unwrap().unwrap()
}

fn read_count_from_storage() -> u32 {
    storage()
        .get_item("clickCounter")
        .ok()
        .flatten()
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(0)
}

fn write_count_to_storage(v: u32) {
    let _ = storage().set_item("clickCounter", &v.to_string());
}

#[wasm_bindgen]
pub fn init_counter() -> u32 {
    let val = read_count_from_storage();
    COUNTER.with(|c| c.set(val));
    val
}

#[wasm_bindgen]
pub fn increment_counter() -> u32 {
    COUNTER.with(|c| {
        let next = c.get().saturating_add(1);
        c.set(next);
        write_count_to_storage(next);
        special_effects(next);
        next
    })
}

pub fn special_effects(count: u32) {
    let audio_21 = web_sys::HtmlAudioElement::new_with_src(
        "https://www.myinstants.com/media/sounds/whats-9-plus-10_i5PRvD4.mp3",
    )
    .unwrap();

    let audio_69 = web_sys::HtmlAudioElement::new_with_src(
        "https://www.myinstants.com/media/sounds/vine-boom.mp3",
    )
    .unwrap();

    match count {
        21 => {
            let _ = audio_21.play();
        }
        // TODO: make the image disappear after 10 seconds and add a sound effect
        67 => {
            let _ = bootstrap_67_kid_image();
        }
        69 => {
            let _ = audio_69.play();
        }
        _ => {}
    }
}

#[wasm_bindgen]
pub fn bootstrap_67_kid_image() -> Result<(), JsValue> {
    let document = window().unwrap().document().expect("document required");

    let img = document
        .create_element("img")?
        .dyn_into::<HtmlImageElement>()?;

    img.set_src("https://i.kym-cdn.com/photos/images/newsfeed/003/128/463/b28");
    img.set_alt("the 67 kid with the blue lasers coming out of his eyes");

    document.body().unwrap().append_child(&img)?;

    Ok(())
}
