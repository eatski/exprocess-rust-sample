pub mod exprocess;
pub mod repository;

pub type Runner = expro::exprocess::Runner<exprocess::AppCore,repository::AppRepository>;

pub fn start(room_id: String,listener: expro::exprocess::Listener<exprocess::AppCore,exprocess::AppState>) -> Runner {
    Runner::start(repository::AppRepository::new(room_id) ,listener)
}