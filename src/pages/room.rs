
use presentation::loading::loading;
use presentation::not_found::not_found;
use yew::prelude::*;
use crate::containers::main::Main;
use js_bridge::{sync_room,Room as RoomData,Phase,start_room,get_your_id,JSFunctionCleaner};
use crate::containers::meeting::{Meeting,MeetingHost};
pub struct Room {
    state: State,
    props: Props,
    link: ComponentLink<Self>,
    on_destroy: JSFunctionCleaner
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

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub room_id: String,
    pub on_error: Callback<()>
}

impl Component for Room {
    type Message = Msg;
    type Properties = Props;

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateRoom(room) => self.state = State::Fetched(room,get_your_id(self.props.room_id.as_str())),
            Msg::RoomNotExists => self.state = State::NotExists,
            Msg::Start => {
                self.state = State::Loading;
                let on_error = self.props.on_error.clone();
                start_room(&self.props.room_id,move || on_error.clone().emit(()));
            }
        };
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        panic!()
    }

    fn destroy(&mut self) {
        self.on_destroy.clean();
    }

    fn view(&self) -> Html {
        match &self.state {
            State::Loading => loading(),
            State::Fetched(room,your_id) => match room.phase {
                Phase::Meeting =>  if !room.is_host {
                    html! {<Meeting room_id=self.props.room_id.clone() on_error=self.props.on_error.clone()/>}
                } else {
                    let start = self.link.callback(|_| Msg::Start);
                    html! {<MeetingHost room_id=self.props.room_id.clone() start=start on_error=self.props.on_error.clone()/>}
                },
                Phase::Started => match your_id {
                    Some(your_id) => html! {
                        <Main 
                            is_host=room.is_host 
                            room_id=self.props.room_id.clone()
                            your_id=your_id.clone()
                            on_error=self.props.on_error.clone()
                        />
                    },
                    None => not_found(),
                }
                
                
            }
            State::NotExists => not_found(),
        }
    }
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let room_id = props.room_id.as_str();
        let callback = link.callback(
             |room : Option<RoomData>| room.map_or(Msg::RoomNotExists, Msg::UpdateRoom)
        );
        let on_error = props.on_error.clone();
        let on_destroy = sync_room(
            room_id, 
            move |room| callback.emit(room),
            move || on_error.clone().emit(())
        );
        Self {
            state: State::Loading,
            props,
            link,
            on_destroy
        }
    }
}