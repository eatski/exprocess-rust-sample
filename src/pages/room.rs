use yew::prelude::*;
use crate::repository::{MembersRepository};

pub struct Room {
    members: Vec<Member>,
    link: Box<ComponentLink<Self>>,
    repository: MembersRepository
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
                self.members = members;
            }
        };
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        todo!()
    }

    fn view(&self) -> Html {
        let members = 
            self.members.iter().map(|member| html! {<li>{&member.name}</li>});
        html! {
            <section>
                <ul>
                    {for members}
                </ul>
            </section>
        }
    }
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            members: Vec::new(),
            link:Box::new(link),
            repository: MembersRepository::new(props.id)
        }
    }
    fn rendered(&mut self, first_render: bool) {
        if first_render {
            let link = self.link.clone();
            self.repository.sync(Box::new(move |members| {
                let members = 
                    members
                    .iter()
                    .map(|member| Member {id:String::from(member.id),name: String::from(member.name)})
                    .collect::<Vec<Member>>();
                link.send_message(Msg::ReplaceMembers(members));
            }));
        }
        
    }
}