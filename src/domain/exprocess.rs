use super::libs::exprocess::ExprocessCore;
use serde::{Deserialize, Serialize};

pub enum AppState {
    Blank,
    Standby(Vec<String>),
    Picked
}

//FIXME ドメインモデルにつけて問題ない？
#[derive(Serialize, Deserialize)]
// #[serde(tag = "type")]
pub enum AppCommand {
    Init(Vec<String>),
    Pick
}

//FIXME ドメインモデルにつけて問題ない？
#[derive(Serialize, Deserialize)]
// #[serde(tag = "type")]
pub enum AppResult {
    Init(Vec<String>),
    Picked
}

pub struct AppCore;
impl ExprocessCore for AppCore {
    type State = AppState;
    type Command = AppCommand;
    type Result = AppResult;

    fn init() -> Self::State {
        AppState::Blank
    }

    fn resolve(_prev: &Self::State,command: &Self::Command) -> Self::Result {
        match command {
            AppCommand::Init(members) => AppResult::Init(members.clone()),
            AppCommand::Pick => todo!(),
        }
    }

    fn reducer(_prev: &Self::State, result: &Self::Result) -> Self::State {
        match result {
            AppResult::Init(members) => AppState::Standby(members.clone()),
            AppResult::Picked => todo!(),
        }
    }
}