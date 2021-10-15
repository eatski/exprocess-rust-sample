use super::libs::exprocess::{Repository,Record};
use super::exprocess::{AppCore};

pub struct AppRepository;

impl Repository<AppCore> for AppRepository {
  
    fn push(&mut self,record: &Record<AppCore>) {
        todo!()
    }

    fn start<L: FnMut(Record<AppCore>)>(listener: L) -> Self {
        Self
    }

}