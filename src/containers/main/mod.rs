use crate::{
    domain::{
        repository::RepositoryError, start, state::AppCommand, state::Member,
        state::Role, Runner,
    },
};

use js_bridge::fetch_members;
use presentation::{set_role::{FormInputs, set_role_guest, set_role_host}, loading::loading, rolled::rolled, standby::{standby, standby_guest}};
use yew::prelude::*;
mod model;
use crate::containers::main::model::{app_state_to_view_state,ViewState,Msg};

pub struct Main {
    runner: Runner,
    state: ViewState,
}

#[derive(Clone, Properties,PartialEq)]
pub struct Props {
    pub is_host: bool,
    pub room_id: String,
    pub your_id: String,
    pub on_error: Callback<()>,
}

impl Component for Main {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props().clone();
        let link_listener = ctx.link().clone();
        let link_on_error = props.on_error.clone();
        let is_host = props.is_host;
        let your_id = props.your_id.clone();
        let runner = start(
            props.room_id.clone(),
            Box::new(move |_, state| {
                let state = app_state_to_view_state(
                    &state, 
                    is_host, 
                    your_id.as_str(),
                    &link_listener.callback(|e| e)
                );
                link_listener.send_message(Msg::UpdateState(state))
            }),
            Box::new(move |err| match err {
                RepositoryError::UnExpected => link_on_error.emit(())
            }),
        );
        Main {
            state: ViewState::Blank,
            runner,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateState(state) => {
                if matches!(state, ViewState::Blank) && ctx.props().is_host {
                    let link = ctx.link().clone();
                    let on_error =  ctx.props().on_error.clone();
                    fetch_members(
                        ctx.props().room_id.as_str(),
                        move |members| {
                            let msg = Msg::PushCommand(AppCommand::Init(
                                members
                                    .iter()
                                    .map(|member| Member {
                                        name: String::from(member.name),
                                        id: String::from(member.id),
                                    })
                                    .collect(),
                            ));
                            link.send_message(msg);
                        },
                        move || on_error.clone().emit(())
                    );
                }
                self.state = state
            }
            Msg::PushCommand(command) => self.runner.dispatch(command)
        };
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        match &self.state {
            ViewState::Blank => loading(),
            ViewState::Setting {members,host_form}=> {
                match host_form {
                    Some(on_submit) => set_role_host(
                        members,
                        &on_submit.reform(
                            |inputs: FormInputs| inputs.into_iter().map(|input| (input.num,Role {name: input.name})).collect() 
                        )
                    ),
                    None => set_role_guest(members),
                }
            }
            ViewState::Standby { members: _ ,host_form,roles} => {
                match host_form {
                    Some(on_submit) => standby(roles, on_submit),
                    None => standby_guest(roles),
                }
            },
            
            ViewState::Picked {member,role , restart_form} => {
                rolled(&member.name,&role.name,restart_form)
            }
            
        }
    }
}
