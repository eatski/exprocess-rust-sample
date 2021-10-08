use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/build/index.js")]
extern "C" {
    #[wasm_bindgen(js_name = "save")]
    pub fn save(roomId: &str,json: &str);

    #[wasm_bindgen(js_name = "sync")]
    pub fn sync(roomId: &str,callback: JsValue);

    #[wasm_bindgen(js_name = "create")]
    pub fn create(callback: JsValue);
}