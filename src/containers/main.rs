use crate::{
    domain::{
        repository::RepositoryError, start, state::AppCommand, state::AppState, state::Member,
        state::PickCommand, state::Role, Runner,
    },
    repository::fetch_members,
};
use presentation::{before_role::{FormInputs, before_roll_guest, before_roll_host}, loading::loading, members::Member as MemberViewModel, rolled::rolled};
use yew::prelude::*;

pub struct Main {
    runner: Runner,
    state: ViewState,
    props: Props,
    link: ComponentLink<Self>,
}

pub enum ViewState {
    Blank,
    Standby {
        members: Vec<MemberViewModel>,
        host_form: Option<Callback<PickCommand>>,
    },
    Picked(Vec<(Member, Role)>)
}

fn app_state_to_view_state(app: &AppState, is_host: bool, your_id: &str,link: &ComponentLink<Main>) -> ViewState {
    match app {
        AppState::Blank => ViewState::Blank,
        AppState::Standby(members) => ViewState::Standby {
            members: members.iter().map(|m| MemberViewModel {name:m.name.clone(),you: m.id.as_str() == your_id}).collect(),
            host_form: if is_host {
                Option::Some(link.callback(|command| Msg::PushCommand(AppCommand::Pick(command))))
            } else {
                Option::None
            },
        },
        AppState::Picked(picked) => ViewState::Picked(picked.picked.iter().cloned().collect()),
    }
}

pub enum Msg {
    UpdateState(ViewState),
    PushCommand(AppCommand),
}

#[derive(Clone, Properties)]
pub struct Props {
    pub is_host: bool,
    pub room_id: String,
    pub your_id: String,
    pub on_error: Callback<()>,
}

impl Component for Main {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let link_listener = link.clone();
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
                    &link_listener
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
            props,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateState(state) => {
                if matches!(state, ViewState::Blank) && self.props.is_host {
                    let link = self.link.clone();
                    let on_error =  self.props.on_error.clone();
                    fetch_members(
                        self.props.room_id.as_str(),
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

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        panic!()
    }

    fn view(&self) -> Html {
        match &self.state {
            ViewState::Blank => loading(),
            ViewState::Standby { members, host_form } => {
                match host_form {
                    Some(on_submit) => before_roll_host(
                        members,
                        &on_submit.reform(
                            |inputs: FormInputs| PickCommand { roles: inputs.into_iter().map(|input| (input.num,Role {name: input.name})).collect() }
                        )
                    ),
                    None => before_roll_guest(members),
                }
            }
            ViewState::Picked(list) => {
                let (you, your_role) = list
                    .iter()
                    .find(move |(member, _)| member.id == self.props.your_id)
                    .expect("No Player Matches");

                rolled(&you.name,&your_role.name)
            }
        }
    }
}
