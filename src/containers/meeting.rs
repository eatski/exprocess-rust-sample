use yew::prelude::*;
use yew::virtual_dom::VNode;
use crate::components::text_input::Input;

use crate::components::loading::loading;
use crate::repository::{register_member,sync_members};
use crate::switch::AppRoute;

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
    link: ComponentLink<Self>,
    on_destroy: Option<Box<dyn FnOnce()>>
}

pub struct Member {
    pub name: String,
    pub id: String,
    pub you: bool,
}

enum State {
    Loading,
    Fetched(Fetched)
}

struct Fetched {
    members:Vec<Member>,
    form:FormState
}
enum FormState {
    Joinable {
        join: Callback<String> 
    },
    Joined,
    Loading
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
        let on_destroy = sync_members(
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
            link,
            on_destroy: Some(on_destroy)
        }
    }
    
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateMember(members) => {
                let form = 
                    if members.iter().any(|m| m.you) { 
                        FormState::Joined
                    } else { 
                        FormState::Joinable {
                            join: self.link.callback(|name| Msg::Join {name})
                        }
                    };
                self.state = State::Fetched( Fetched {members,form} );
                true
            },
            Msg::Join { name } => {
                match &mut self.state {
                    State::Fetched(fetched)  => {
                        fetched.form = FormState::Loading;
                    },
                    _ => panic!()
                }
                register_member(
                    self.props.room_id.as_str(),
                    name.as_str()
                );
                true
            },
            
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        panic!()
    }

    fn destroy(&mut self) {
        self.on_destroy.take().map(|call| call());
    }

    fn view(&self) -> Html {
        match &self.state {
            State::Loading => loading(),
            State::Fetched (fetched) => {
                let title = match &fetched.form {
                    FormState::Joinable { join: _ } => "JOIN US!",
                    FormState::Joined => "Waiting host...",
                    FormState::Loading => "...",
                };
                let form_html = match &fetched.form {
                    FormState::Joinable {join} => html! { 
                        <Input on_submit=join button="Join"/>
                    },
                    FormState::Joined => html! {},
                    FormState::Loading => loading(),
                };
                html! { 
                    <>
                        <h2> {title} </h2>
                        {members_view(&fetched.members)}
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
    on_destroy: Option<Box<dyn FnOnce()>>
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
        let on_destroy = sync_members(
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
            state: StateHost::Loading,
            on_destroy: Some(on_destroy)
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
        // propsのせいで発火する
        true
    }

    fn destroy(&mut self) {
        self.on_destroy.take().map(|call| call());
    }

    fn view(&self) -> Html {
        match &self.state {
            StateHost::Loading => loading(),
            StateHost::Fetched { members } => {
                let route = AppRoute::Room(self.props.room_id.clone()).into_route().route;
                let onclick = self.props.start.reform(|_| ());
                html! { 
                    <>
                        <h2> {"Start when you have all the members!"} </h2>
                        <a href=route>{"Copy this and share URL!"}</a>
                        {members_view(members)}
                        <button onclick=onclick>{"Start!"}</button>
                    </>
                }
            },
        }
        
    }
}
