mod js_bridge;
use mytil::Cleaner;
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{self};
use names::{Generator, Name};
pub use js_bridge::{CleanableJSFunction,JSFunctionCleaner};

#[derive(Serialize, Deserialize)]
struct MemberJSON<'a> {
    pub name: &'a str,
    pub id: &'a str,
    pub you: bool
}



pub fn register_member<OE: FnOnce() + 'static>(room_id: &str,name: &str,on_error: OE) {
    js_bridge::register_member(room_id,name,on_error)
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
    CleanableJSFunction::from(js_bridge::sync_member(
        room_id, 
        Closure::wrap(json_callback).into_js_value(),
        Closure::wrap(on_error).into_js_value(),
    )).into()
}

pub fn fetch_members<CB: FnOnce(Vec<Member>) + 'static,OE: FnOnce() + 'static>(room_id: &str,callback:CB,on_error: OE) {
    js_bridge::fetch_members( 
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
    js_bridge::create_room(
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
    let callback = Box::new(move |room: Option<String>| {
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
    let on_error = Box::new(on_error);
    js_bridge::sync_room(
        room_id, 
        callback,
        on_error
    )
}

pub fn start_room<OE: FnOnce() + 'static>(room_id: &str,on_error: OE) {
    js_bridge::start_room(room_id,Closure::once_into_js(on_error));
}

pub struct RecordPushIO<'a> {
    pub id: &'a str,
    pub command: &'a str,
    pub result: &'a str
}

pub fn push_record<OE: FnOnce() + 'static>(room_id: &str,record: RecordPushIO,on_error: OE) {
    js_bridge::push_record(
        room_id,
        record.id,
        record.command,
        record.result,
        Closure::once_into_js(on_error)
    )
}

pub fn sync_record_update<F: FnMut(String) + 'static,E: FnMut() + 'static>(room_id: &str,listener: F,on_error: E) -> Cleaner<CleanableJSFunction> {
    js_bridge::sync_record_update(room_id, Box::new(listener),Box::new(on_error))
}

pub fn get_your_id(room_id: &str) -> Option<String> {
    js_bridge::get_your_id(room_id)
}