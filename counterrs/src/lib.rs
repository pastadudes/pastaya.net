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

//! # counterrs is a replacement for the old Javascript based counter  
//! ## It extends on the base of the original counter adding:  
//! 1: An anticheat (technically)  
//! 2: Special effects for certain numbers: (plays vine boom at 69 for example)  
//! 3: Faster and type safe. (it can also be rust propaganda)
//! Of course it has drawbacks, namely that old browsers (around internet explorer age) can't  
//! run it  
//! It also isn't a complete 1:1 version of the old counter because of how it uses local_storage  
//! instead of cookies
// HEY! special_effects() is a STUB!! PLEASE HELP EXPAND IT!

// my goal is for this to ACTUALLY be a good clicker game
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlImageElement, window};
mod audio;
mod counter;
mod effects;
use crate::effects::special_effects;
use wasm_bindgen_test::*;

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

/// This function bootstraps an image and puts it in the DOM  
/// Examples:  
/// ```rust
/// let _ = bootstrap_image(
///     "https://media1.tenor.com/m/WUWygJ0Fwz8AAAAd/jago33-slot-machine.gif", // This is what image to display
///         Some("slot machine go BRRRRRRRRRRRR".to_string()), // Alt
///         6000, // Duration in milliseconds
///         None, // We don't use a class
///    );
/// ```
/// WE HAVE TO USE `.to_string()` BECAUSE THE FUNCTION ONLY WANTS A `Option<String>`!

#[wasm_bindgen]
pub fn bootstrap_image(
    src: &str,
    alt: Option<String>,
    duration: u32,
    class: Option<String>,
) -> Result<(), JsValue> {
    let document = window().unwrap().document().expect("document required");

    let img = document
        .create_element("img")?
        .dyn_into::<HtmlImageElement>()?;

    img.set_src(src);

    if let Some(a) = alt {
        img.set_alt(&a);
    }
    if let Some(c) = class {
        img.set_class_name(&c);
    }

    document.body().unwrap().append_child(&img)?;
    let img_clone = img.clone();
    wasm_bindgen_futures::spawn_local(async move {
        TimeoutFuture::new(duration).await;
        #[allow(clippy::let_unit_value)]
        let _ = img_clone.remove();
    });

    Ok(()) // 👍
}
// unit tests start here

// ------------------------------------------------------------------------------------------------------------------------------------------------------------
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
#[allow(dead_code)]
fn test_bootstrap_image_inserts_img() {
    let _ = bootstrap_image(
        "https://example.com/test.gif",
        Some("alt text".to_string()),
        1000,
        Some("test-class".to_string()),
    );

    let document = window().unwrap().document().unwrap();
    let body = document.body().unwrap();

    // Enter image.
    let img = body
        .last_child()
        .unwrap()
        .dyn_into::<HtmlImageElement>()
        .unwrap();

    // is it good???
    assert!(img.src().contains("https://example.com/test.gif"));
    assert_eq!(img.alt(), "alt text");
    assert_eq!(img.class_name(), "test-class");
}

#[wasm_bindgen_test(async)]
#[allow(dead_code)]
async fn test_bootstrap_image_removes_after_duration() {
    let _ = bootstrap_image(
        "https://example.com/test.gif",
        None,
        100, // only 0.1s cuz uhhhhhhhh
        None,
    );

    let document = window().unwrap().document().unwrap();
    let body = document.body().unwrap();

    assert!(body.query_selector("img").unwrap().is_some());

    gloo_timers::future::TimeoutFuture::new(200).await;

    assert!(body.query_selector("img").unwrap().is_none());
}
