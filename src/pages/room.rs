use yew::prelude::*;
use crate::components::loading::loading;
use crate::containers::main::Main;
use crate::repository::{sync_room,Room as RoomData,Phase,start_room};
use crate::containers::meeting::{Meeting,MeetingHost};
pub struct Room {
    state: State,
    props: Props,
    link: ComponentLink<Self>
}
enum State {
    Loading,
    Fetched(RoomData),
    NotExists
}

pub enum Msg {
    UpdateRoom(RoomData),
    RoomNotExists,
    Start
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
            Msg::UpdateRoom(room) => self.state = State::Fetched(room),
            Msg::RoomNotExists => self.state = State::NotExists,
            Msg::Start => start_room(&self.props.room_id)
        };
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        panic!()
    }

    fn view(&self) -> Html {
        let content = match &self.state {
            State::Loading => loading(),
            State::Fetched(room) => match room.phase {
                Phase::Meeting =>  if !room.is_host {
                    html! {<Meeting room_id=self.props.room_id.clone()/>}
                } else {
                    let start = self.link.callback(|_| Msg::Start);
                    html! {<MeetingHost room_id=self.props.room_id.clone() start=start/>}
                },
                Phase::Started => html! {<Main is_host=room.is_host room_id=self.props.room_id.clone()/>},
            }
            State::NotExists => html! { 
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
        let callback = link.callback(|room : Option<RoomData>| room.map_or(Msg::RoomNotExists, Msg::UpdateRoom));
        //FIXME: unsync
        sync_room(
            room_id, 
            Box::new(move |room| callback.emit(room))
        );
        Self {
            state: State::Loading,
            props,
            link
        }
    }
}