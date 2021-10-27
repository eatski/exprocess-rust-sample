use yew::prelude::*;
use crate::components::{loading::loading,not_found::not_found};
use crate::containers::main::Main;
use crate::repository::{sync_room,Room as RoomData,Phase,start_room,get_your_id};
use crate::containers::meeting::{Meeting,MeetingHost};
pub struct Room {
    state: State,
    props: Props,
    link: ComponentLink<Self>,
    on_destroy: Box<dyn FnMut()>
}

type YourId = Option<String>;
enum State {
    Loading,
    Fetched(RoomData,YourId),
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
            Msg::UpdateRoom(room) => self.state = State::Fetched(room,get_your_id(self.props.room_id.as_str())),
            Msg::RoomNotExists => self.state = State::NotExists,
            Msg::Start => start_room(&self.props.room_id)
        };
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        panic!()
    }

    fn destroy(&mut self) {
        (self.on_destroy)();
        self.on_destroy = Box::new(||());
    }

    fn view(&self) -> Html {
        let content = match &self.state {
            State::Loading => loading(),
            State::Fetched(room,your_id) => match room.phase {
                Phase::Meeting =>  if !room.is_host {
                    html! {<Meeting room_id=self.props.room_id.clone()/>}
                } else {
                    let start = self.link.callback(|_| Msg::Start);
                    html! {<MeetingHost room_id=self.props.room_id.clone() start=start/>}
                },
                Phase::Started => match your_id {
                    Some(your_id) => html! {
                        <Main 
                            is_host=room.is_host 
                            room_id=self.props.room_id.clone()
                            your_id=your_id.clone()
                        />
                    },
                    None => not_found(),
                }
                
                
            }
            State::NotExists => not_found(),
        };
        html! {
            <section>
                {content}
            </section>
        }
    }
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let room_id = props.room_id.as_str();
        let callback = link.callback(
             |room : Option<RoomData>| room.map_or(Msg::RoomNotExists, Msg::UpdateRoom)
        );
        let on_destroy = sync_room(
            room_id, 
            Box::new(move |room| callback.emit(room))
        );
        Self {
            state: State::Loading,
            props,
            link,
            on_destroy
        }
    }
}