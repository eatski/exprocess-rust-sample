use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/build/index.js")]
extern "C" {
    #[wasm_bindgen(js_name = "registerMember")]
    pub fn register_member(room_id: &str,name: &str);

    #[wasm_bindgen(js_name = "syncMember")]
    pub fn sync_member(room_id: &str,callback: JsValue);

    #[wasm_bindgen(js_name = "createRoom")]
    pub fn create_room(hostName:&str,callback: JsValue);

    #[wasm_bindgen(js_name = "syncRoom")]
    fn sync_room_bridge(room_id: &str,callback: JsValue);

    #[wasm_bindgen(js_name = "startRoom")]
    pub fn start_room(room_id: &str);
}


pub fn sync_room(room_id: &str,callback: Box<dyn Fn(Option<String>)>) {
    let callback = Closure::into_js_value(Closure::wrap(callback));
    sync_room_bridge(room_id,callback);
}
