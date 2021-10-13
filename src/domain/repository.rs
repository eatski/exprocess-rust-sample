use super::libs::exprocess::{Repository,Record};
use super::exprocess::{AppCommand, AppCore, AppResult};

pub struct AppRepository;

impl Repository<AppCore> for AppRepository {
    fn start(listener: Box<dyn Fn(Record<AppCommand,AppResult>)>) -> Self {
        todo!()
    }

    fn push(&mut self,record: &Record<AppCommand,AppResult>) {
        todo!()
    }
}