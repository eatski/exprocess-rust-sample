use crate::repository::{RecordIO, push_record, sync_record_update};

use super::libs::exprocess::{Repository,Record};
use super::exprocess::{AppCore};

pub struct AppRepository {
    room_id: String
}

fn to_io(record: &Record<AppCore>) -> RecordIO {
    todo!()
}

fn to_model<'a>(record: &'a RecordIO) -> Record<'a,AppCore> {
    todo!()
}

impl Repository<AppCore> for AppRepository {
  
    fn push(&mut self,record: &Record<AppCore>) {
        push_record(self.room_id.as_str(),to_io(record))
    }

    fn start(arg:String /* FIXME */, mut listener: Box<dyn FnMut(Record<AppCore>)>) -> Self {
        sync_record_update(arg.as_str(), move |record| listener(to_model(&record)));
        Self {
            room_id: arg
        }
    }
}