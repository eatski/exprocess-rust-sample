
use mytil::{Cleanable, Cleaner};
use wasm_bindgen::prelude::*;
use js_sys::Function;
use serde::{Deserialize, Serialize};
use serde_json::{self};
use names::{Generator, Name};

#[wasm_bindgen()]
extern "C" {
    #[wasm_bindgen(js_name = "registerMember",js_namespace = ["window","_wasm_js_bridge"])]
    fn register_member_bridge(room_id: &str,name: &str,on_error: JsValue);

    #[wasm_bindgen(js_name = "syncMember",js_namespace = ["window","_wasm_js_bridge"])]
    fn sync_member_bridge(room_id: &str,callback: JsValue,on_error: JsValue) -> Function;

    #[wasm_bindgen(js_name = "fetchMembers",js_namespace = ["window","_wasm_js_bridge"])]
    fn fetch_members_bridge(room_id: &str,callback: JsValue,on_error: JsValue);

    #[wasm_bindgen(js_name = "createRoom",js_namespace = ["window","_wasm_js_bridge"])]
    fn create_room_bridge(room_id: &str,hostName:&str,callback: JsValue,on_error: JsValue);

    #[wasm_bindgen(js_name = "syncRoom",js_namespace = ["window","_wasm_js_bridge"])]
    fn sync_room_bridge(room_id: &str,callback: JsValue,on_error: JsValue) -> Function;

    #[wasm_bindgen(js_name = "startRoom",js_namespace = ["window","_wasm_js_bridge"])]
    fn start_room_bridge(room_id: &str,on_error: JsValue);

    #[wasm_bindgen(js_name = "pushRecord",js_namespace = ["window","_wasm_js_bridge"])]
    fn push_record_bridge(room_id: &str,record_id: &str, command: &str, result: &str,on_error: JsValue);

    #[wasm_bindgen(js_name = "syncRecordUpdate",js_namespace = ["window","_wasm_js_bridge"])]
    fn sync_record_update_bridge(room_id: &str,callback: JsValue,on_error: JsValue) -> Function;

    #[wasm_bindgen(js_name = "getYourId",js_namespace = ["window","_wasm_js_bridge"])]
    fn get_your_id_bridge(room_id: &str) -> Option<String>;
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



pub fn register_member<OE: FnOnce() + 'static>(room_id: &str,name: &str,on_error: OE) {
    register_member_bridge(room_id,name, Closure::once_into_js(Box::new(on_error)))
}

pub fn sync_record_update<F: FnMut(String) + 'static,E: FnMut() + 'static>(room_id: &str, callback: F,on_error: E) -> JSFunctionCleaner {
    let callback: Box<dyn FnMut(String)> = Box::new(callback);
    let callback = Closure::wrap( callback).into_js_value();
    let on_error: Box<dyn FnMut()>  = Box::new(on_error);
    let on_error = Closure::wrap(on_error).into_js_value();
    CleanableJSFunction::from(sync_record_update_bridge(room_id,callback,on_error)).into()
}

#[derive(Serialize, Deserialize)]
struct MemberJSON<'a> {
    pub name: &'a str,
    pub id: &'a str,
    pub you: bool
}


fn json_to_members<'a>(json:&'a str) -> Vec<Member<'a>> {
    let members_json : Vec<MemberJSON> = serde_json::from_str(json).expect("JSON Parse Error");
    members_json
            .iter()
            .map(|member| Member {id:member.id,name: member.name, you: member.you})
            .collect::<Vec<Member>>()
}

pub fn sync_members<CB: FnMut(Vec<Member>) + 'static,OE: FnMut() + 'static>(room_id: &str,mut callback:CB,on_error: OE) -> JSFunctionCleaner {
    let json_callback : Box<dyn FnMut(String)>= Box::new(
        move |json:String| callback(json_to_members(&json))
    );
    let on_error : Box<dyn FnMut()> = Box::new(on_error);
    CleanableJSFunction::from(sync_member_bridge(
        room_id, 
        Closure::wrap(json_callback).into_js_value(),
        Closure::wrap(on_error).into_js_value(),
    )).into()
}

pub fn fetch_members<CB: FnOnce(Vec<Member>) + 'static,OE: FnOnce() + 'static>(room_id: &str,callback:CB,on_error: OE) {
    fetch_members_bridge( 
        room_id, 
        Closure::once_into_js(move |json:String| callback(json_to_members(json.as_str()))),
        Closure::once_into_js (on_error),
    )
}

pub struct Member<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub you: bool
}

pub fn create_room<CB: FnOnce() + 'static,OE: FnOnce() + 'static>(hostname: &str,callback : CB,on_error: OE) -> String {
    let mut generator = Generator::with_naming(Name::Numbered);
    let room_id = generator.next().unwrap();
    create_room_bridge(
        room_id.as_str(),
        hostname,
        Closure::once_into_js(callback),
        Closure::once_into_js(on_error)
    );
    room_id
}

pub enum Phase {
    Meeting,
    Started
}


pub struct Room {
    pub phase: Phase,
    pub is_host: bool  
}

#[derive(Serialize, Deserialize)]
pub struct RoomJSON<'a> {
    pub phase: &'a str,
    pub is_host: bool  
}

pub fn sync_room<CB: FnMut(Option<Room>) + 'static, OE: FnMut() + 'static>(room_id: &str,mut callback:CB,on_error: OE) -> Cleaner<CleanableJSFunction> {
    let callback: Box<dyn FnMut(Option<String>)> = Box::new(move |room| {
        let room = room.map(|room| -> Room {
            let room: RoomJSON = serde_json::from_str(room.as_str()).expect("JSON Parse Error");
            Room { 
                phase: match room.phase {
                    "MEETING" => Phase::Meeting,
                    "STARTED" => Phase::Started,
                    invalid => panic!("Invalid Value {}",invalid)
                }, 
                is_host: room.is_host 
            }
        });
        callback(room);
    });
    let on_error: Box<dyn FnMut()> = Box::new(on_error);
    let callback = Closure::wrap(callback).into_js_value();
    let on_error = Closure::wrap(on_error).into_js_value();
    CleanableJSFunction::from(sync_room_bridge(room_id,callback,on_error)).into()
}

pub fn start_room<OE: FnOnce() + 'static>(room_id: &str,on_error: OE) {
    start_room_bridge(room_id,Closure::once_into_js(on_error));
}

pub struct RecordPushIO<'a> {
    pub id: &'a str,
    pub command: &'a str,
    pub result: &'a str
}

pub fn push_record<OE: FnOnce() + 'static>(room_id: &str,record: RecordPushIO,on_error: OE) {
    push_record_bridge(
        room_id,
        record.id,
        record.command,
        record.result,
        Closure::once_into_js(on_error)
    )
}

pub fn get_your_id(room_id: &str) -> Option<String> {
    get_your_id_bridge(room_id)
}