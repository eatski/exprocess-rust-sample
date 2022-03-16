use yew::prelude::*;
use presentation::{members::Member as MemberViewModel};

use crate::domain::state::{ Role, AppState, AppCommand, SetRole, Started, Member};

pub enum ViewState {
    Blank,
    Setting {
        members: Vec<MemberViewModel>,
        host_form: Option<Callback<SetRole>>,
    },
    Standby {
        members: Vec<MemberViewModel>,
        host_form: Option<Callback<()>>
    },
    Picked(MemberViewModel, Role)
}

fn member_to_viewmodel(member: &Member,your_id: &str) -> MemberViewModel{
    MemberViewModel {
        name: member.name.clone(),
        you: member.id == your_id,
    }
}

pub fn app_state_to_view_state(app: &AppState, is_host: bool, your_id: &str,callback: &Callback<Msg>) -> ViewState {
    match app {
        AppState::Blank => ViewState::Blank,
        AppState::Setting(members) => {
            ViewState::Setting {
                members: members.values().map(|m| member_to_viewmodel(m,your_id)).collect(),
                host_form: is_host.then(|| callback.reform(Msg::PushCommand).reform(AppCommand::SetRole)),
            }
        },
        AppState::Started(setting, started) => {
            match started {
                Started::Standby => ViewState::Standby {
                    members: setting.members.values().map(|m| member_to_viewmodel(m,your_id)).collect(),
                    host_form: is_host.then(|| callback.reform(Msg::PushCommand).reform(|_| AppCommand::Pick)),
                },
                Started::Picked(picked) => {
                    let (member,role) = picked.iter().map(|(m,r)| (setting.members.get(m).unwrap(),setting.roles.get(r).unwrap())).map(|(m,(_,r))| (member_to_viewmodel(m,your_id),r.clone())).find(|(m,_)| m.you).unwrap();
                    ViewState::Picked(member,role)
                }
            }
        },
    }
}

pub enum Msg {
    UpdateState(ViewState),
    PushCommand(AppCommand),
}