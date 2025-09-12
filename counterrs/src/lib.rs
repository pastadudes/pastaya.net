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
//! 3: Faster and type safe. (it can also be rust propaganda)
//! Of course it has drawbacks, namely that old browsers (around internet explorer age) can't  
//! run it  
//! It also isn't a complete 1:1 version of the old counter because of how it uses local_storage  
//! instead of cookies
// HEY! special_effects() is a STUB!! PLEASE HELP EXPAND IT!

// my goal is for this to ACTUALLY be a good clicker game
use gloo_timers::future::TimeoutFuture;
use std::cell::Cell;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlImageElement, window};

thread_local! {
    static COUNTER: Cell<u32> = Cell::new(0);
    static IS_PAUSED: Cell<bool> = Cell::new(false); // for future reference...
}
#[wasm_bindgen]
/// Sets IS_PAUSED to true.
pub fn pause_counter() {
    IS_PAUSED.with(|p| p.set(true));
}

// Function to resume the counter (start incrementing and writing to storage again)
#[wasm_bindgen]
/// Sets IS_PAUSED to false.
pub fn resume_counter() {
    IS_PAUSED.with(|p| p.set(false));
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

// MORE OPTIMZATION!!!
thread_local! {
    static AUDIO_1_1: web_sys::HtmlAudioElement =
        web_sys::HtmlAudioElement::new_with_src("/assets/audio/1-part1.wav").unwrap();

    static AUDIO_1_2: web_sys::HtmlAudioElement =
        web_sys::HtmlAudioElement::new_with_src("/assets/audio/1-part2.wav").unwrap();

    static AUDIO_21: web_sys::HtmlAudioElement =
        web_sys::HtmlAudioElement::new_with_src("https://www.myinstants.com/media/sounds/whats-9-plus-10_i5PRvD4.mp3")
            .unwrap();

    static AUDIO_42: web_sys::HtmlAudioElement =
        web_sys::HtmlAudioElement::new_with_src("/assets/audio/42.wav").unwrap();

    static AUDIO_67: web_sys::HtmlAudioElement =
        web_sys::HtmlAudioElement::new_with_src("https://www.myinstants.com//media/sounds/67_SQlv2Xv.mp3")
            .unwrap();

    static AUDIO_69: web_sys::HtmlAudioElement =
        web_sys::HtmlAudioElement::new_with_src("https://www.myinstants.com/media/sounds/vine-boom.mp3")
            .unwrap();

    static AUDIO_420: web_sys::HtmlAudioElement =
    web_sys::HtmlAudioElement::new_with_src("https://www.myinstants.com/media/sounds/im-on-that-good-kush-high-quality.mp3")
        .unwrap();

    static AUDIO_666: web_sys::HtmlAudioElement =
    web_sys::HtmlAudioElement::new_with_src("https://www.myinstants.com/media/sounds/susto-666.mp3")
        .unwrap();

    static AUDIO_777: web_sys::HtmlAudioElement =
    web_sys::HtmlAudioElement::new_with_src("https://www.myinstants.com/media/sounds/slotmachine.mp3")
        .unwrap();

    static AUDIO_9000: web_sys::HtmlAudioElement =
    web_sys::HtmlAudioElement::new_with_src("https://www.myinstants.com/media/sounds/its_over_9000.mp3")
        .unwrap();

    static AUDIO_80085: web_sys::HtmlAudioElement =
        web_sys::HtmlAudioElement::new_with_src("/assets/audio/80085.wav").unwrap();


}

#[wasm_bindgen]
/// Increments the counter.  
/// It also checks if IS_PAUSED is true or not.  
/// If IS_PAUSED is true then it doesn't increment, pretty simple
pub fn increment_counter() -> u32 {
    if IS_PAUSED.with(|p| p.get()) {
        return COUNTER.with(|c| c.get()); // Don't increment if paused
    }
    COUNTER.with(|c| {
        let next = c.get().saturating_add(1);
        c.set(next);
        write_count_to_storage(next);
        // we actually don't want audio_nya to be in thread_local! because it doesn't stack
        // (basically uhh im saying that it doesn't overlap with itself)
        if let Ok(audio) = web_sys::HtmlAudioElement::new_with_src(
            "https://www.myinstants.com/media/sounds/fnf-kapi-nyaw-sound-effect.mp3",
        ) {
            let _ = audio.play();
        }
        wasm_bindgen_futures::spawn_local(special_effects(next));
        next
    })
}

/// Plays a set of special effects on a certain "number"  
/// All numbers that have a special effect:  
/// 1. 1
/// 2. 21
/// 3. 42
/// 4. 67
/// 5. 69
/// 6. 420
/// 7. 666
/// 8. 777
pub async fn special_effects(count: u32) {
    match count {
        1 => {
            AUDIO_1_1.with(|a| {
                let _ = a.play();
            });
            TimeoutFuture::new(2000).await;
            AUDIO_1_2.with(|a| {
                let _ = a.play();
            });
        }
        21 => AUDIO_21.with(|a| {
            let _ = a.play();
        }),
        42 => AUDIO_42.with(|a| {
            let _ = a.play();
        }),
        67 => {
            AUDIO_67.with(|a| {
                let _ = a.play();
            });
            let _ = bootstrap_image(
                "https://i.kym-cdn.com/photos/images/newsfeed/003/128/463/b28",
                Some("the 67 kid with the blue lasers coming out of his eyes".to_string()),
                3000,
                Some("half".to_string()),
            );
        }
        69 => AUDIO_69.with(|a| {
            let _ = a.play();
        }),
        420 => AUDIO_420.with(|a| {
            let _ = a.play();
        }),
        666 => AUDIO_666.with(|a| {
            let _ = a.play();
        }),
        777 => {
            let _ = bootstrap_image(
                "https://media1.tenor.com/m/WUWygJ0Fwz8AAAAd/jago33-slot-machine.gif",
                Some("slot machine go BRRRRRRRRRRRR".to_string()),
                6000,
                Some("slotmachine".to_string()),
            );

            // TODO: reverse the order and make the audio play after slot machine has 3 7's
            AUDIO_777.with(|a| {
                let _ = a.play();
            });
        }
        9000 => AUDIO_9000.with(|a| {
            let _ = a.play();
        }),
        80085 => AUDIO_80085.with(|a| {
            let _ = a.play();
        }),

        _ => {}
    }
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
    // match alt {
    //     Some(a) => img.set_alt(&a),
    //     None => {}
    // }
    // match class {
    //     Some(c) => img.set_class_name(&c),
    //     None => {}
    // }
    // don't mind me
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

/// This function is only meant for testing purposes, however I have no objections to anyone that  
/// tries this  
/// DOES WRITE TO STORAGE!!!
#[wasm_bindgen]
pub fn set_counter(v: u32) {
    COUNTER.with(|c| c.set(v));
    write_count_to_storage(v);
    wasm_bindgen_futures::spawn_local(special_effects(v));
}

/// Resets the counter. Pretty self explantory  
/// Probably considered the same as "set_counter(0)"
#[wasm_bindgen]
pub fn reset_counter() {
    COUNTER.with(|c| c.set(0));
    write_count_to_storage(0);
}
