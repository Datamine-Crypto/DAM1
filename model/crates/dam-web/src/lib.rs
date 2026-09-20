#![deny(non_snake_case)]
#![deny(unreachable_patterns)]
#![forbid(unknown_lints)]

use spec::contexts::training::vocabulary::WORD_STEPS;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn load(weights: &[u8], classes: &str) -> Result<usize, String> {
    dam_page::load(weights, classes)
}

#[wasm_bindgen]
pub fn state(bytes: &[u8]) -> Result<usize, String> {
    dam_page::state(bytes)
}

#[wasm_bindgen]
pub fn context() -> Result<Vec<u8>, String> {
    dam_page::context()
}

#[wasm_bindgen]
pub fn restore(bytes: &[u8]) -> Result<(), String> {
    dam_page::restore(bytes)
}

#[wasm_bindgen]
pub fn forget() -> Result<(), String> {
    dam_page::forget()
}

#[wasm_bindgen]
pub fn read(text: &str) -> Result<String, String> {
    dam_page::read(text, WORD_STEPS)
}

#[wasm_bindgen]
pub fn describe(text: &str) -> Result<String, String> {
    dam_page::described(text)
}

#[wasm_bindgen]
pub fn kinds(names: &str) -> Result<String, String> {
    dam_page::kinds(names)
}

#[wasm_bindgen]
pub fn world() -> Result<String, String> {
    dam_page::world()
}

#[wasm_bindgen]
pub fn settings() -> Result<String, String> {
    dam_page::settings(WORD_STEPS)
}
