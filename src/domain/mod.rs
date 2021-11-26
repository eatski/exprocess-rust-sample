pub mod state;
pub mod repository;

pub type Runner = exprocess::client::Runner<state::AppCore,repository::RepositoryError>;

pub fn start(room_id: String,listener: exprocess::client::Listener<state::AppCore,state::AppState>,on_error: Box<dyn FnMut(repository::RepositoryError)>) -> Runner {
    let repo = repository::AppRepository::new(room_id);
    let repo = exprocess::directly::DirectlyDispatch::wrap(repo);
    Runner::start(repo,listener,on_error)
}