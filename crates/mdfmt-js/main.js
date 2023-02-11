import wasm from "./pkg/mdfmt_js_bg.wasm";
import { __wbg_set_wasm } from "./pkg/mdfmt_js_bg.js";
import * as bg from "./pkg/mdfmt_js_bg.js";

export * from "./pkg/mdfmt_js_bg.js";

export async function load() {
    const buf = Buffer.from(wasm, "base64");
    const res = await WebAssembly.instantiate(buf, {
        "./mdfmt_js_bg.js": bg,
    });
    __wbg_set_wasm(res.instance.exports);
}
