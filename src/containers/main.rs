use yew::prelude::*;
use crate::{domain::{Runner, start, exprocess::AppState,exprocess::AppCommand, exprocess::Member,exprocess::Role}, repository::{fetch_members}};

pub struct Main {
    runner:Runner,
    state: ViewState,
    props: Props,
    link: ComponentLink<Self>
}

pub enum ViewState {
    Blank,
    Standby { members:Vec<String> },
    Picked { result: Vec<(Member,Role)>}
}

fn app_state_to_view_state(app:&AppState) -> ViewState {
    match app {
        AppState::Blank => ViewState::Blank,
        AppState::Standby(members) => ViewState::Standby { 
            members:members.iter().map(|m| m.name.clone()).collect()
        },
        AppState::Picked(picked) => ViewState::Picked {
            result: todo!()
        },
    }
}

pub enum Msg {
    UpdateState(ViewState),
    PushCommand(AppCommand)
} 

#[derive(Clone,Eq,PartialEq,Properties)]
pub struct Props {
    pub is_host: bool,
    pub room_id: String
}

impl Component for Main {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let link_cloned = link.clone();
        let runner = start (
            props.room_id.clone(),
            Box::new(move |_,state| link_cloned.send_message(Msg::UpdateState(app_state_to_view_state(state))))
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
            Msg::UpdateState(state) => self.state = state,
            Msg::PushCommand(command) => self.runner.dispatch(command),
        };
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        panic!()
    }

    fn rendered(&mut self, _first_render: bool) {
        if _first_render && self.props.is_host {
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
    }

    fn view(&self) -> Html {
        match &self.state {
            ViewState::Blank => html! {
                "Started"
            },
            ViewState::Standby {members} => {
                html! {
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
                    
                }
            },
            ViewState::Picked { result } => todo!(),
        }
        
    }
}