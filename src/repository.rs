use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/repository.js")]
extern "C" {
    #[wasm_bindgen(js_name = "getPayload")]
    pub fn get_payload() -> String;

    #[wasm_bindgen(js_name = "getPayloadLater")]
    pub fn get_payload_later(payload_callback: JsValue);
}