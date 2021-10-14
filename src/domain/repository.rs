use super::libs::exprocess::{Repository,Record};
use super::exprocess::{AppCore};

pub struct AppRepository;

impl Repository<AppCore> for AppRepository {
    fn start(listener: Box<dyn Fn(Record<AppCore>)>) -> Self {
        todo!()
    }

    fn push(&mut self,record: &Record<AppCore>) {
        todo!()
    }
}