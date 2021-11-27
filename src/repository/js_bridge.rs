use mytil::{Cleanable, Cleaner};
use wasm_bindgen::prelude::*;
use js_sys::Function;

#[wasm_bindgen()]
extern "C" {
    #[wasm_bindgen(js_name = "registerMember",js_namespace = ["window","_wasm_js_bridge"])]
    fn register_member_bridge(room_id: &str,name: &str,on_error: JsValue);

    #[wasm_bindgen(js_name = "syncMember",js_namespace = ["window","_wasm_js_bridge"])]
    pub fn sync_member(room_id: &str,callback: JsValue,on_error: JsValue) -> Function;

    #[wasm_bindgen(js_name = "fetchMembers",js_namespace = ["window","_wasm_js_bridge"])]
    pub fn fetch_members(room_id: &str,callback: JsValue,on_error: JsValue);

    #[wasm_bindgen(js_name = "createRoom",js_namespace = ["window","_wasm_js_bridge"])]
    pub fn create_room(room_id: &str,hostName:&str,callback: JsValue,on_error: JsValue);

    #[wasm_bindgen(js_name = "syncRoom",js_namespace = ["window","_wasm_js_bridge"])]
    fn sync_room_bridge(room_id: &str,callback: JsValue,on_error: JsValue) -> Function;

    #[wasm_bindgen(js_name = "startRoom",js_namespace = ["window","_wasm_js_bridge"])]
    pub fn start_room(room_id: &str,on_error: JsValue);

    #[wasm_bindgen(js_name = "pushRecord",js_namespace = ["window","_wasm_js_bridge"])]
    pub fn push_record(room_id: &str,record_id: &str, command: &str, result: &str,on_error: JsValue);

    #[wasm_bindgen(js_name = "syncRecordUpdate",js_namespace = ["window","_wasm_js_bridge"])]
    fn sync_record_update_bridge(room_id: &str,callback: JsValue,on_error: JsValue) -> Function;

    #[wasm_bindgen(js_name = "getYourId",js_namespace = ["window","_wasm_js_bridge"])]
    pub fn get_your_id(room_id: &str) -> Option<String>;
}

pub fn register_member<OE: FnOnce() + 'static>(room_id: &str,name: &str,on_error: OE) {
    register_member_bridge(room_id,name, Closure::once_into_js(Box::new(on_error)))
}

pub fn sync_room(room_id: &str,callback: Box<dyn FnMut(Option<String>)>,on_error: Box<dyn FnMut()>) -> JSFunctionCleaner {
    let callback = Closure::wrap(callback).into_js_value();
    let on_error = Closure::wrap(on_error).into_js_value();
    CleanableJSFunction::from(sync_room_bridge(room_id,callback,on_error)).into()
}

pub fn sync_record_update(room_id: &str, callback: Box<dyn FnMut(String)>,on_error: Box<dyn FnMut()>) -> JSFunctionCleaner {
    let callback = Closure::wrap(callback).into_js_value();
    let on_error = Closure::wrap(on_error).into_js_value();
    CleanableJSFunction::from(sync_record_update_bridge(room_id,callback,on_error)).into()
}

pub struct CleanableJSFunction {
    js_function: Function
}

impl From<Function> for CleanableJSFunction {
    fn from(js_function: Function) -> Self {
        Self { js_function }
    }
}

impl Cleanable for CleanableJSFunction {
    fn clean(self) {
        self.js_function.call0(&JsValue::NULL).expect("JS function call err");
    }
}

pub type JSFunctionCleaner = Cleaner<CleanableJSFunction>;