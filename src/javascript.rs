use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/repository.js")]
extern "C" {
    #[wasm_bindgen(js_name = "getPayload")]
    pub fn get_payload() -> String;

    #[wasm_bindgen(js_name = "getPayloadLater")]
    pub fn get_payload_later(payload_callback: JsValue);

    #[wasm_bindgen(js_name = "save")]
    pub fn save(roomId: &str,namespace: &str, json: &str);

    #[wasm_bindgen(js_name = "sync")]
    pub fn sync(roomId: &str,namespace: &str, callback: JsValue);
}