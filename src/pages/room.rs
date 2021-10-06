use yew::prelude::*;
use wasm_bindgen::prelude::*;
use crate::repository;

pub struct Room {
    id: String,
    members: Vec<Member>,
    link: Box<ComponentLink<Self>>
}

pub struct Member<> {
    pub name: String,
    pub id: String,
}

pub enum Msg {
    AddMember(String)
}

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub id: String
}

impl Component for Room {
    type Message = Msg;
    type Properties = Props;

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.members.push(match msg {
            Msg::AddMember(text) => Member {id: "TODO".to_string(),name: text}
        });
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
                <h2>{ self.id.clone() }</h2>
                <ul>
                    {for members}
                </ul>
            </section>
        }
    }
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            id: props.id,
            members: Vec::new(),
            link:Box::new(link)
        }
    }
    fn rendered(&mut self, first_render: bool) {
        if first_render {
            let link = self.link.clone();
            let cl = Box::new(move |text:String| {
                link.send_message(Msg::AddMember(text));
            }) as Box<dyn Fn(String)>;
            let cl = Closure::wrap(cl);
            let callback = Closure::into_js_value(cl);
            repository::get_payload_later(callback);
        }
        
    }
}