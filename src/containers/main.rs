use yew::prelude::*;
use crate::{components::loading::loading, containers::host_form::HostForm, domain::{Runner, start, state::AppCommand, state::{AppStateContent}, state::Member, state::PickCommand, state::Role}, repository::{fetch_members}};

pub struct Main {
    runner:Runner,
    state: ViewState,
    props: Props,
    link: ComponentLink<Self>
}

pub enum ViewState {
    Loading,
    Blank,
    Standby { 
        members:Vec<String>,
        host_form: Option<Callback<PickCommand>>
    },
    Picked (Vec<(Member,Role)>)
}


fn app_state_to_view_state(app:&AppStateContent,is_host: bool, link: &ComponentLink<Main>) -> ViewState {
    match app {
        AppStateContent::Blank => ViewState::Blank,
        AppStateContent::Standby(members) => ViewState::Standby { 
            members:members.iter().map(|m| m.name.clone()).collect(),
            host_form: if is_host { 
                Option::Some(
                    link.callback(|command| Msg::PushCommand(AppCommand::Pick(command)))
                ) 
            } else { 
                Option::None 
            }
        },
        AppStateContent::Picked(picked) => ViewState::Picked (
            picked.picked.iter().map(|(m,r)| (m.clone(),r.clone())).collect()
        ),
    }
}

pub enum Msg {
    UpdateState(ViewState),
    PushCommand(AppCommand)
} 

#[derive(Clone,Eq,PartialEq,Properties)]
pub struct Props {
    pub is_host: bool,
    pub room_id: String,
    pub your_id: String
}

impl Component for Main {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let link_cloned = link.clone();
        let is_host = props.is_host;
        let runner = start (
            props.room_id.clone(),
            Box::new(move |_,state| {
                let state = app_state_to_view_state(&state.content,is_host,&link_cloned);
                link_cloned.send_message(Msg::UpdateState(state))
            })
        );
        Main {
            state: ViewState::Blank,
            runner,
            props,
            link
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateState(state) => { 
                if matches!(state,ViewState::Blank) && self.props.is_host {
                    let link = self.link.clone();
                    fetch_members(self.props.room_id.as_str(),Box::new(move |members| {
                        let msg = Msg::PushCommand(
                            AppCommand::Init(members.iter()
                                .map(|member| Member { name: String::from(member.name),id: String::from(member.id)} )
                                .collect()
                            )
                        );
                        link.send_message(msg);
                    }));
                }
                self.state = state
            },
            Msg::PushCommand(command) => self.runner.dispatch(command),
        };
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        panic!()
    }

    fn view(&self) -> Html {
        match &self.state {
            ViewState::Blank => html! {
                "Started"
            },
            ViewState::Standby {members, host_form} => {
                let host_form_view = match host_form {
                    Some(on_submit) => html! {
                        <section>
                            <h2>{"Roles"}</h2>
                            <HostForm on_submit=on_submit members_num=members.len()/> 
                        </section>
                    },
                    None => html! {},
                };
                html! {
                    <>
                        <section>
                            <h2>{"Joined Members"}</h2>
                            <ul>
                                {for members.iter().map(|member| {
                                    html! {
                                        <li>{member}</li>
                                    }
                                })}
                            </ul>
                            
                        </section>
                        {host_form_view}
                    </>
                    
                    
                }
            },
            ViewState::Picked ( list ) => {
                let (you,your_role) = list
                    .iter()
                    .find(move |(member,_)| member.id == self.props.your_id)
                    .expect("No Player Matches");
                
                html! {
                    <section>
                        <h2>{format!("You({}) is [{}]",you.name,your_role.name)}</h2>
                    </section>
                }
            },
            ViewState::Loading => loading(),
        }
        
    }
}