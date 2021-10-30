use wasm_bindgen::prelude::*;
use js_sys::Function;

#[wasm_bindgen()]
extern "C" {
    #[wasm_bindgen(js_name = "registerMember",js_namespace = ["window","_wasm_js_bridge"])]
    pub fn register_member(room_id: &str,name: &str);

    #[wasm_bindgen(js_name = "syncMember",js_namespace = ["window","_wasm_js_bridge"])]
    pub fn sync_member(room_id: &str,callback: JsValue) -> Function;

    #[wasm_bindgen(js_name = "fetchMembers",js_namespace = ["window","_wasm_js_bridge"])]
    pub fn fetch_members(room_id: &str,callback: JsValue);

    #[wasm_bindgen(js_name = "createRoom",js_namespace = ["window","_wasm_js_bridge"])]
    pub fn create_room(room_id: &str,hostName:&str,callback: JsValue);

    #[wasm_bindgen(js_name = "syncRoom",js_namespace = ["window","_wasm_js_bridge"])]
    fn sync_room_bridge(room_id: &str,callback: JsValue) -> Function;

    #[wasm_bindgen(js_name = "startRoom",js_namespace = ["window","_wasm_js_bridge"])]
    pub fn start_room(room_id: &str);

    #[wasm_bindgen(js_name = "pushRecord",js_namespace = ["window","_wasm_js_bridge"])]
    pub fn push_record(room_id: &str,record_id: &str, command: &str, result: &str);

    #[wasm_bindgen(js_name = "syncRecordUpdate",js_namespace = ["window","_wasm_js_bridge"])]
    fn sync_record_update_bridge(room_id: &str,callback: JsValue) -> Function;

    #[wasm_bindgen(js_name = "getYourId",js_namespace = ["window","_wasm_js_bridge"])]
    pub fn get_your_id(room_id: &str) -> Option<String>;
}

pub fn sync_room(room_id: &str,callback: Box<dyn Fn(Option<String>)>) -> Box<dyn FnOnce()> {
    let callback = Closure::into_js_value(Closure::wrap(callback));
    jsfunction_to_function(sync_room_bridge(room_id,callback))
}

pub fn sync_record_update(room_id: &str, callback: Box<dyn FnMut(String)>) -> Box<dyn FnOnce()> {
    let callback = Closure::into_js_value(Closure::wrap(callback));
    jsfunction_to_function(sync_record_update_bridge(room_id,callback))
}

pub fn jsfunction_to_function(f: Function) -> Box<dyn FnOnce()>{
    Box::new(move || {f.call0(&JsValue::NULL).expect("JS function call err");})
}
