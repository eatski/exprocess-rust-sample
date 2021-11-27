use std::{cell::RefCell, rc::Rc};

use crate::repository::{push_record, RecordPushIO, sync_record_update};
use serde_json::{self,Error as SerdeErr};
use serde::{Deserialize};

use exprocess::client::{Record, RecordSync, Repository};
use super::state::{AppCommand, AppCore, AppResult};

pub enum RepositoryError {
    UnExpected
}

impl From<SerdeErr> for RepositoryError {
    fn from(err: SerdeErr) -> Self {
        log::error!("{}",err.to_string());
        RepositoryError::UnExpected
    }
}

pub struct AppRepository {
    room_id: String,
    unsync_fn: Option<Box<dyn FnOnce()>>
}

impl AppRepository {
    pub fn new(room_id: String) -> Self {
        Self {
            room_id,
            unsync_fn: None
        }
    }
}

#[derive(Deserialize)]
pub struct RecordDesirailizeIO {
    id: String,
    command: AppCommand,
    result: AppResult
}

impl Repository<AppCore,RepositoryError> for AppRepository {

    fn push(&mut self,record: Record<AppCore>,on_error: Box<dyn FnOnce(RepositoryError)>) {

        match (serde_json::to_string(&record.result),serde_json::to_string(&record.command)) {
            (Ok(result_json), Ok(command_json)) => {
                let io = RecordPushIO {
                    id: record.id.as_str(),
                    result: result_json.as_str(),
                    command: command_json.as_str()
                };
                push_record(
                    self.room_id.as_str(),
                    io,
                    || on_error(RepositoryError::UnExpected)
                );
            },
            (_, Err(err)) => on_error(err.into()),
            (Err(err), _) => on_error(err.into()),
        }
    }

    fn sync(&mut self,mut listener: Box<dyn FnMut(Vec<RecordSync<AppCore>>)>,on_error: Box<dyn FnMut(RepositoryError)>) {
        let on_error = Rc::new(RefCell::new(on_error));
        let on_error_callback = on_error.clone();
        self.unsync_fn = Some(sync_record_update(
            self.room_id.as_str(), 
            move |json| {
            let result : Result<Vec<RecordDesirailizeIO>,_> = serde_json::from_str(&json);
            match result {
                Ok(records) => {
                    listener(
                        records.iter()
                        .map(|record| RecordSync {id: record.id.as_str(), result: &record.result, command: &record.command}) 
                        .collect()
                    );
                },
                Err(err) => on_error_callback.borrow_mut()(err.into()),
            }
        },
        move || on_error.borrow_mut()(RepositoryError::UnExpected)
    ));
    }

    fn unsync(&mut self) {
        self.unsync_fn.take().map(|call| call());
    }
}

