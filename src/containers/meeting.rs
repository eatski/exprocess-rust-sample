use yew::prelude::*;
use yew::virtual_dom::VNode;
use crate::components::join_form::Input;

use crate::components::loading::loading;
use crate::repository::{register_member,sync_members};

// Common

fn members_view(members:&Vec<Member>)-> VNode {
    let members = members.iter().map(|member| html! {
        <li>
            <span>{&member.name}</span>
            {if member.you {html! {<span>{"⇨YOU"}</span>}} else {html! {}}}
        </li>
    });
    html! {
        <ul>{for members}</ul>
    }
}

// for Guest
pub struct Meeting {
    props: Props,
    state: State,
    link: ComponentLink<Self>
}

pub struct Member {
    pub name: String,
    pub id: String,
    pub you: bool,
}

enum State {
    Loading,
    Fetched {
        members:Vec<Member>,
        form:FormState
    }
}
enum FormState {
    Joinable {
        join: Callback<String> 
    },
    Joined,
    JoinedAsHost {
        start: Callback<()> 
    }
}

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub room_id : String,
    pub host: Option<Callback<()>>
}

pub enum Msg {
    UpdateMember(Vec<Member>),
    Join {name:String}
}

impl Component for Meeting {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let update = link.callback(Msg::UpdateMember);
        sync_members(
            props.room_id.as_str(), 
            Box::new(
                move |members| {
                    let members = 
                        members
                        .iter()
                        .map(|member| Member {id:String::from(member.id),name: String::from(member.name), you: member.you})
                        .collect::<Vec<Member>>();
                    update.emit(members)
                }
            )
        );
        Self {
            props,
            state: State::Loading,
            link
        }
    }
    
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateMember(members) => {
                let form = 
                    if members.iter().any(|m| m.you) { 
                        self.props.host
                            .clone()
                            .map_or(FormState::Joined , |start|  FormState::JoinedAsHost {start})    
                    } else { 
                        FormState::Joinable {
                            join: self.link.callback(|name| Msg::Join {name})
                        }
                    };
                self.state = State::Fetched {members,form};
                true
            },
            Msg::Join { name } => {
                register_member(
                    self.props.room_id.as_str(),
                    name.as_str()
                );
                false
            },
            
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        panic!()
    }

    fn view(&self) -> Html {
        match &self.state {
            State::Loading => loading(),
            State::Fetched { members, form } => {
                let form_html = match form {
                    FormState::Joinable {join} => html! { 
                        <Input on_submit=join button="Join"/>
                    },
                    FormState::Joined => html! {},
                    FormState::JoinedAsHost { start } => {
                        let onclick = start.reform(|_| ());
                        html! { 
                            <button onclick=onclick>{"Start"}</button>
                        }
                    }
                };
                html! { 
                    <>
                        <h2> { "Meeting"} </h2>
                        {members_view(members)}
                        {form_html}
                    </>
                }
            },
        }
    }
}

// for Host

pub struct MeetingHost {
    props: PropsHost,
    state: StateHost,
}

#[derive(Clone, Properties)]
pub struct PropsHost {
    pub room_id : String,
    pub start: Callback<()>
}

enum StateHost {
    Loading,
    Fetched {
        members:Vec<Member>,
    }
}

pub enum MsgHost {
    UpdateMember(Vec<Member>)
}

impl Component for MeetingHost {

    type Message = MsgHost;
    type Properties = PropsHost;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let update = link.callback(MsgHost::UpdateMember);
        sync_members(
            props.room_id.as_str(), 
            Box::new(
                move |members| {
                    let members = 
                        members
                        .iter()
                        .map(|member| Member {id:String::from(member.id),name: String::from(member.name), you: member.you})
                        .collect::<Vec<Member>>();
                    update.emit(members)
                }
            )
        );
        Self {
            props,
            state: StateHost::Loading
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            MsgHost::UpdateMember(members) => {
                self.state = StateHost::Fetched {members};
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        panic!()
    }

    fn view(&self) -> Html {
        match &self.state {
            StateHost::Loading => loading(),
            StateHost::Fetched { members } => {
                let onclick = self.props.start.reform(|_| ());
                html! { 
                    <>
                        <h2> {"Host"} </h2>
                        {members_view(members)}
                        <button onclick=onclick>{"Start"}</button>
                    </>
                }
            },
        }
        
    }
}
