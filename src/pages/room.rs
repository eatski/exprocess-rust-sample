use yew::prelude::*;
use crate::components::loading::loading;
use crate::repository::{sync_room,Phase};
use crate::containers::meeting::{Meeting};
pub struct Room {
    state: State,
    props: Props
}
enum State {
    Loading,
    Meeting,
    Started,
    NotExist
}


pub enum Msg {
    MovePhase(Phase)
}

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub room_id: String
}

impl Component for Room {
    type Message = Msg;
    type Properties = Props;

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MovePhase(phase) => self.state = match phase {
                Phase::RoomNotExists => State::NotExist,
                Phase::Meeting => State::Meeting,
                Phase::Started => State::Started,
            },
        };
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        panic!()
    }

    fn view(&self) -> Html {
        let content = match &self.state {
            State::Loading => loading(),
            State::Meeting => html! {<Meeting room_id=self.props.room_id.clone()/>},
            State::Started => html! { 
                <h2> { "Gathered" } </h2>
            },
            State::NotExist => html! { 
                <h2> { "404" } </h2>
            },
        };
        html! {
            <section>
                {content}
            </section>
        }
    }
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let room_id = props.room_id.as_str();
        //FIXME: unsync
        sync_room(
            room_id, 
            Box::new(move |phase| link.send_message(Msg::MovePhase(phase)))
        );
        Self {
            state: State::Loading,
            props
        }
    }
}