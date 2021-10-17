use crate::repository::{RecordIO, push_record, sync_record_update};
use serde_json::{self};

use super::libs::exprocess::{Repository,Record};
use super::exprocess::{AppCore};

pub struct AppRepository {
    room_id: String
}

impl AppRepository {
    pub fn new(room_id: String) -> Self {
        Self {
            room_id
        }
    }
}

impl Repository<AppCore> for AppRepository {
  
    fn push(&mut self,record: &Record<AppCore>) {
        push_record(
            self.room_id.as_str(),
            RecordIO {
                id: String::from(record.id),
                result: serde_json::to_string(record.result).expect("JSON Serialize Err"),
                command: serde_json::to_string(record.command).expect("JSON Serialize Er")
            }
        )
    }

    fn sync(&mut self,mut listener: Box<dyn FnMut(Record<AppCore>)>) {
        sync_record_update(self.room_id.as_str(), move |record| {
            let record = Record {
                id: record.id.as_str(),
                command: &serde_json::from_str(record.command.as_str()).expect("JSON Parse Err"),
                result: &serde_json::from_str(record.result.as_str()).expect("JSON Parse Err"),
            };
            listener(record);
        });
    }
    
}