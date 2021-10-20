pub mod exprocess;
pub mod repository;

pub type Runner = expro::exprocess::Runner<exprocess::AppCore>;

pub fn start(room_id: String,listener: expro::exprocess::Listener<exprocess::AppCore,exprocess::AppState>) -> Runner {
    let repo = expro::stacked::DirectlyDispatch::wrap(repository::AppRepository::new(room_id));
    Runner::start(repo,listener)
}