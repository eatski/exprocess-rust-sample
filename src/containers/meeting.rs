use yew::prelude::*;
use crate::components::join_form::Input;

use crate::components::loading::loading;
use crate::repository::{register_member,sync_members};

pub struct Meeting {
    props: Props,
    state: State,
    link: ComponentLink<Self>
}

pub struct Member {
    pub name: String,
    pub id: String,
    pub you: bool
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
    Joined
}

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub room_id : String
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
                        FormState::Joined 
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
                let members_html = 
                members.iter().map(|member| html! {
                    <li>
                        <span>{&member.name}</span>
                        {if member.you {html! {<span>{"⇨YOU"}</span>}} else {html! {}}}
                    </li>
                }
            );
            let form_html = match form {
                FormState::Joinable {join} => {
                    html! { 
                        <Input on_submit=join button="Join"/>
                    }
                }
                FormState::Joined => html! {},
            };
            html! { 
                <>
                    <h2> { "Meeting"} </h2>
                    <ul>
                        { for members_html } 
                    </ul>
                    {form_html}
                </>
            }
            },
        }
    }
}

