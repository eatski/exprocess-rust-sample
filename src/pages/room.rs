use yew::prelude::*;

pub struct Room {
    pub id: String,
    pub members: Vec<Member>
}

pub struct Member {
    pub name: String,
    pub id: String
}

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub id: String
}

impl Component for Room {
    type Message = ();
    type Properties = Props;

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        todo!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        todo!()
    }

    fn view(&self) -> Html {
        html! {
            <div>{ self.id.clone() }</div>
        }
    }

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            id: props.id,
            members: Vec::new()
        }
    }
}