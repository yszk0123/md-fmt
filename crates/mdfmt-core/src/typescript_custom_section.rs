use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(typescript_custom_section)]
const TYPESCRIPT_CUSTOM_SECTION: &'static str = r#"
export type Utc = any;
export type Value = any;
export type DateTime<T> = string;
export type NaiveDate = string;
export type FlattenNode = string;
"#;
