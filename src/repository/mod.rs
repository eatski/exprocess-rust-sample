mod js_bridge;
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{self};

#[derive(Serialize, Deserialize)]
struct MemberJSON<'a> {
    pub name: &'a str,
    pub id: &'a str,
    pub you: bool
}

pub fn register_member(room_id: &str,name: &str) {
    js_bridge::register_member(room_id,name)
}

pub fn sync_members(room_id: &str,callback:Box<dyn Fn(Vec<Member>) -> ()>) {
    let json_callback : Box<dyn Fn(String)>= Box::new(move |json:String| {
        let members_json : Vec<MemberJSON> = serde_json::from_str(json.as_str()).expect("JSON Parse Error");
        let members = members_json
            .iter()
            .map(|member| Member {id:member.id,name: member.name, you: member.you})
            .collect::<Vec<Member>>();
        callback(members);
    });
    js_bridge::sync_member(
        room_id, 
        Closure::into_js_value(Closure::wrap(json_callback))
    )
    
}

pub struct Member<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub you: bool
}

pub fn create_room(callback : Box<dyn FnOnce(String)>) {
    js_bridge::create_room(
        Closure::into_js_value(Closure::once (callback))
    )
}

pub enum Phase {
    RoomNotExists,
    Meeting,
    Started
}

pub fn sync_room(room_id: &str,callback:Box<dyn Fn(Phase) -> ()>) {
    let callback : Box<dyn Fn(Option<String>)>= Box::new(move |phase| {
        let phase = match phase {
            Option::Some(phase) => match phase.as_str() {
                "MEETING" => Phase::Meeting,
                "STARTED" => Phase::Started,
                invalid => panic!("Invalid Phase Value : {}",invalid)
            },
            None => Phase::RoomNotExists,
        };
        callback(phase);
    });
    js_bridge::sync_room(
        room_id, 
        Closure::into_js_value(Closure::once (callback))
    );
}

