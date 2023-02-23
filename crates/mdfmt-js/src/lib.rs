use mdfmt_core::format as f;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn format(name: &str) -> String {
    f(name).unwrap_or_else(|_| name.to_string())
}
