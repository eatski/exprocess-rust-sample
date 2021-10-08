use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/build/index.js")]
extern "C" {

    #[wasm_bindgen(js_name = "save")]
    pub fn save(roomId: &str,namespace: &str, json: &str);

    #[wasm_bindgen(js_name = "sync")]
    pub fn sync(roomId: &str,namespace: &str, callback: JsValue);
}