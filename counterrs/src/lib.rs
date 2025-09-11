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

//! counterrs is a replacement for the old Javascript based counter  
//! It extends on the base of the original counter adding:  
//! 1: An anticheat (technically)  
//! 2: Special effects for certain numbers: (plays vine boom at 69 for example)  
//! 3: Faster and type safe.  
//! Of course it has drawbacks, namely that old browsers (around internet explorer age) can't  
//! run it  
//! It also isn't a complete 1:1 version of the old counter because of how it uses local_storage  
//! instead of cookies
// HEY! special_effects() is a STUB!! PLEASE HELP EXPAND IT!

// my goal is for this to ACTUALLY be a good clicker game
use std::cell::Cell;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlImageElement, window};

#[warn(clippy::missing_const_for_thread_local)]
thread_local! {
    static COUNTER: Cell<u32> = Cell::new(0);
}

// we use local storage instead of cookies since cookies have a shitton of limits
fn storage() -> web_sys::Storage {
    window().unwrap().local_storage().unwrap().unwrap()
}

// This function returns how many times a person has clicked the "nyabtn" in pastaya.net
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
    let audio_67 = web_sys::HtmlAudioElement::new_with_src(
        "https://www.myinstants.com//media/sounds/67_SQlv2Xv.mp3",
    )
    .unwrap();
    let audio_42 = web_sys::HtmlAudioElement::new_with_src("/assets/audio/42.wav").unwrap();
    let audio_80085 = web_sys::HtmlAudioElement::new_with_src("/assets/audio/80085.wav").unwrap();

    match count {
        21 => {
            let _ = audio_21.play();
        }
        42 => {
            let _ = audio_42.play();
        }
        // TODO: make the image disappear after 10 seconds and add a sound effect
        67 => {
            let _ = bootstrap_67_kid_image();
            let _ = audio_67.play();
        }
        69 => {
            let _ = audio_69.play();
        }
        80085 => {
            let _ = audio_80085.play();
        }
        _ => {}
    }
}

/// This function basically shows the 67 kid image when called
#[wasm_bindgen]
pub fn bootstrap_67_kid_image() -> Result<(), JsValue> {
    let document = window().unwrap().document().expect("document required");

    let img = document
        .create_element("img")?
        .dyn_into::<HtmlImageElement>()?;

    img.set_src("https://i.kym-cdn.com/photos/images/newsfeed/003/128/463/b28");
    img.set_alt("the 67 kid with the blue lasers coming out of his eyes");
    // img.set_class_name("half");
    // TODO: Set correct width and height for 67 kid image (so its smaller)

    document.body().unwrap().append_child(&img)?;
    // TODO: make it so that it disappears after about 10 seconds

    Ok(()) // 👍
}

/// This function is only meant for testing purposes, however I have no objections to anyone that
/// tries this
/// DOES WRITE TO STORAGE!!!
#[wasm_bindgen]
pub fn set_counter(v: u32) {
    COUNTER.with(|c| c.set(v));
    write_count_to_storage(v);
    special_effects(v);
}

/// Resets the counter. Pretty self explantory
/// Probably considered the same as "set_counter(0)"
#[wasm_bindgen]
pub fn reset_counter() {
    COUNTER.with(|c| c.set(0));
    write_count_to_storage(0);
}
