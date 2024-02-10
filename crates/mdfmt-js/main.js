import wasm from "./pkg/mdfmt_js_bg.wasm";
import loadWasm from "./pkg/mdfmt_js.js";

export * from "./pkg/mdfmt_js.js";

export async function load() {
    await loadWasm(Buffer.from(wasm, "base64"));
}
