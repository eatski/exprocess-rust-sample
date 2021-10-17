

pub mod libs;
pub mod exprocess;
pub mod repository;

pub type Runner = libs::exprocess::Runner<exprocess::AppCore,repository::AppRepository>;

pub fn start(room_id: String,listener: libs::exprocess::Listener<exprocess::AppCore>) -> Runner {
    Runner::start(repository::AppRepository::new(room_id) ,listener)
}