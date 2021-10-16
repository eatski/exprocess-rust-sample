use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/build/index.js")]
extern "C" {
    #[wasm_bindgen(js_name = "registerMember")]
    pub fn register_member(room_id: &str,name: &str);

    #[wasm_bindgen(js_name = "syncMember")]
    pub fn sync_member(room_id: &str,callback: JsValue);

    #[wasm_bindgen(js_name = "fetchMembers")]
    pub fn fetch_members(room_id: &str,callback: JsValue);

    #[wasm_bindgen(js_name = "createRoom")]
    pub fn create_room(hostName:&str,callback: JsValue);

    #[wasm_bindgen(js_name = "syncRoom")]
    fn sync_room_bridge(room_id: &str,callback: JsValue);

    #[wasm_bindgen(js_name = "startRoom")]
    pub fn start_room(room_id: &str);

    #[wasm_bindgen(js_name = "pushRecord")]
    pub fn push_record(room_id: &str,record_id: &str, command: &str, result: &str);

    #[wasm_bindgen(js_name = "syncRecordUpdate")]
    fn sync_record_update_bridge(room_id: &str,callback: JsValue);
}


pub fn sync_room(room_id: &str,callback: Box<dyn Fn(Option<String>)>) {
    let callback = Closure::into_js_value(Closure::wrap(callback));
    sync_room_bridge(room_id,callback);
}

pub fn sync_record_update(room_id: &str, callback: Box<dyn FnMut(String,String,String)>) {
    let callback = Closure::into_js_value(Closure::wrap(callback));
    sync_record_update_bridge(room_id,callback);
}
