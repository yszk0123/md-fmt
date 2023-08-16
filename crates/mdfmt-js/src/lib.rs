use mdfmt_core as core;
pub use mdfmt_core::model;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn format(input: &str) -> String {
    core::format(input).unwrap_or_else(|_| input.to_string())
}

#[wasm_bindgen]
pub fn parse(input: &str) -> Option<Note> {
    core::parse(input).ok()
}
