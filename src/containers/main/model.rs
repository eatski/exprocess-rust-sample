use yew::prelude::*;
use presentation::{members::Member as MemberViewModel,roles::Role as RoleViewModel};

use crate::domain::state::{ Role, AppState, AppCommand, SetRole, Started, Member};

pub enum ViewState {
    Blank,
    Setting {
        members: Vec<MemberViewModel>,
        host_form: Option<Callback<SetRole>>,
    },
    Standby {
        members: Vec<MemberViewModel>,
        roles: Vec<(usize,RoleViewModel)>,
        host_form: Option<Callback<()>>
    },
    Picked {
        member:MemberViewModel ,
        role: RoleViewModel,
        restart_form: Option<Callback<()>>
    }
}

fn member_to_viewmodel(member: &Member,your_id: &str) -> MemberViewModel{
    MemberViewModel {
        name: member.name.clone(),
        you: member.id == your_id,
    }
}

fn role_to_viewmodel(role: &Role) -> RoleViewModel{
    RoleViewModel {
        name: role.name.clone(),
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
                    roles: setting.roles.values().map(|(idx,role)| (*idx,role_to_viewmodel(role))).collect(),
                    host_form: is_host.then(|| callback.reform(Msg::PushCommand).reform(|_| AppCommand::Pick)),
                },
                Started::Picked(picked) => {
                    let (member,role) = picked.iter()
                        .map(|(m,r)| (setting.members.get(m).unwrap(),setting.roles.get(r).unwrap()))
                        .map(|(m,(_,r))| (member_to_viewmodel(m,your_id),role_to_viewmodel(r))).find(|(m,_)| m.you)
                        .unwrap();
                    ViewState::Picked {
                        member,
                        role,
                        restart_form: is_host.then(|| callback.reform(Msg::PushCommand).reform(|_| AppCommand::Restart)),
                    }
                }
            }
        },
    }
}

pub enum Msg {
    UpdateState(ViewState),
    PushCommand(AppCommand),
}