use super::libs::exprocess::ExprocessCore;

pub enum AppState {

}
pub enum AppCommand {

}
pub enum AppResult {

}

pub struct AppCore;
impl ExprocessCore for AppCore {
    type State = AppState;
    type Command = AppCommand;
    type Result = AppResult;

    fn init() -> Self::State {
        todo!()
    }

    fn resolve(prev: &Self::State,command: &Self::Command) -> Self::Result {
        todo!()
    }

    fn reducer(prev: &Self::State, result: &Self::Result) -> Self::State {
        todo!()
    }
}