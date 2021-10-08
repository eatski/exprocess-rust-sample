use yew::prelude::*;
use crate::repository::{MembersRepository};
use crate::components::input::Input;
pub struct Room {
    state: RoomState,
    submit: Callback<String>
}
enum RoomState {
    Loading,
    Meeting {
        members: Vec<Member>,
        form: FormState,
    },
    Gathered {
        members: Vec<Member>,
    }
}
enum FormState {
    Joinable,
    Joined
}

pub struct Member {
    pub name: String,
    pub id: String,
}

pub enum Msg {
    ReplaceMembers(Vec<Member>)
}

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub id: String
}

impl Component for Room {
    type Message = Msg;
    type Properties = Props;

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ReplaceMembers(members) => {
                self.state = RoomState::Meeting {
                    members:members,
                    form: FormState::Joinable 
                };
            }
        };
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        todo!()
    }

    fn view(&self) -> Html {
        let content = match &self.state {
            RoomState::Loading => html! { <div>{"Loading"}</div>},
            RoomState::Meeting {members,form} => {
                let members_html = members.iter().map(|member| html! {<li>{&member.name}</li>});
                let form_html = match form {
                    FormState::Joinable => {
                        html! { 
                            <Input on_submit=self.submit.clone()/>
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
            RoomState::Gathered { members} => {
                let members_html = members.iter().map(|member| html! {<li>{&member.name}</li>});
                html! { 
                    <>
                        <h2> { "Gathered" } </h2>
                        <ul>
                            { for members_html } 
                        </ul>
                    </>
                }
            }
        };
        html! {
            <section>
                {content}
            </section>
        }
    }
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let repo = MembersRepository::new(props.id);
        //FIXME: unsync
        repo.sync(Box::new(move |members| {
            let members = 
                members
                .iter()
                .map(|member| Member {id:String::from(member.id),name: String::from(member.name)})
                .collect::<Vec<Member>>();
            link.send_message(Msg::ReplaceMembers(members));
        }));
        Self {
            state: RoomState::Loading,
            submit: Callback::from(move |name| repo.save(name))
        }
    }
}