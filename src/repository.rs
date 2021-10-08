use crate::javascript;
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{self};

pub struct MembersRepository {
    room_id: String
}

#[derive(Serialize, Deserialize)]
struct MemberJSON<'a> {
    pub name: &'a str,
    pub id: &'a str,
}

impl MembersRepository {
    pub fn new(room_id: String) -> Self {
        MembersRepository {
            room_id:room_id
        }
    }
    pub fn save(&self,name:String) {
        javascript::save(self.room_id.as_str(), "members", name.as_str())
    }
    pub fn sync(&self,callback:Box<dyn Fn(Vec<Member>) -> ()>) {
        let json_callback : Box<dyn Fn(String)>= Box::new(move |json:String| {
            let members_json : Vec<MemberJSON> = serde_json::from_str(json.as_str()).expect("JSON Parse Error");
            let members = members_json
                .iter()
                .map(|member| Member {id:member.id,name: member.name})
                .collect::<Vec<Member>>();
            callback(members);
        });
        javascript::sync(
            self.room_id.as_str(), 
            "members", 
            Closure::into_js_value(Closure::wrap(json_callback))
        )
        
    }
}

pub struct Member<'a> {
    pub id: &'a str,
    pub name: &'a str
}

