use crate::{read_count_from_storage, special_effects, wasm_bindgen, write_count_to_storage};
use std::cell::Cell;

thread_local! {
    pub static COUNTER: Cell<u32> = Cell::new(0);
    pub static IS_PAUSED: Cell<bool> = Cell::new(false); // for future reference...
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

#[wasm_bindgen]
pub fn init_counter() -> u32 {
    let val = read_count_from_storage();
    COUNTER.with(|c| c.set(val));
    val
}

/// Increments the counter.  
/// It also checks if IS_PAUSED is true or not.  
/// If IS_PAUSED is true then it doesn't increment, pretty simple
#[wasm_bindgen]
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
    let document = web_sys::window()
        .expect("NO WINDOW!")
        .document()
        .expect("bruh no document git gud");
    let element = document
        .get_element_by_id("counter")
        .expect("bro your website doesn't work its missing the counter!!");
    element.set_text_content(Some("0"));
}
