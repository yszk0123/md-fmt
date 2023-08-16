pub use mdfmt_core::model;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn format(input: &str) -> String {
    mdfmt_core::format(input).unwrap_or_else(|_| input.to_string())
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Note")]
    pub type Note;
}

#[wasm_bindgen]
pub fn parse(input: &str) -> Result<Note, JsError> {
    let note = mdfmt_core::parse(input).map_err(to_js_error)?;
    let value = serde_wasm_bindgen::to_value(&note).map_err(to_js_error)?;
    Ok(Note::from(value))
}

#[wasm_bindgen]
pub fn stringify(input: Note) -> Result<String, JsError> {
    let value = serde_wasm_bindgen::from_value(JsValue::from(input)).map_err(to_js_error)?;
    let note = mdfmt_core::stringify(&value).map_err(to_js_error)?;
    Ok(note)
}

fn to_js_error(err: impl std::fmt::Display) -> JsError {
    JsError::new(&err.to_string())
}
