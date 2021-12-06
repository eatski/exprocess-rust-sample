use yew::prelude::*;
use presentation::{members::Member as MemberViewModel};

use crate::domain::state::{PickCommand, Member, Role, AppState, AppCommand};

pub enum ViewState {
    Blank,
    Standby {
        members: Vec<MemberViewModel>,
        host_form: Option<Callback<PickCommand>>,
    },
    Picked(Vec<(Member, Role)>)
}

pub fn app_state_to_view_state(app: &AppState, is_host: bool, your_id: &str,callback: &Callback<Msg>) -> ViewState {
    match app {
        AppState::Blank => ViewState::Blank,
        AppState::Standby(members) => ViewState::Standby {
            members: members.iter().map(|m| MemberViewModel {name:m.name.clone(),you: m.id.as_str() == your_id}).collect(),
            host_form: is_host.then(|| callback.reform(Msg::PushCommand).reform(AppCommand::Pick)),
        },
        AppState::Picked(picked) => ViewState::Picked(picked.picked.clone()),
    }
}

pub enum Msg {
    UpdateState(ViewState),
    PushCommand(AppCommand),
}