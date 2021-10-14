use super::libs::exprocess::{Repository,Record};
use super::exprocess::{AppCore};

pub struct AppRepository;

impl Repository<AppCore> for AppRepository {
  

    fn push(&mut self,record: &Record<AppCore>) {
        todo!()
    }

    fn start(listener: Box<dyn FnMut(Record<AppCore>)>) -> Self {
        todo!()
    }
}