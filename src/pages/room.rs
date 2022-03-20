
use presentation::loading::loading;
use presentation::not_found::not_found;
use yew::prelude::*;
use crate::containers::main::Main;
use js_bridge::{sync_room,Room as RoomData,Phase,start_room,get_your_id,JSFunctionCleaner};
use crate::containers::meeting::{Meeting,MeetingHost};
pub struct Room {
    state: State,
    props: Props,
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

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub room_id: String,
    pub on_error: Callback<()>
}

impl Component for Room {
    type Message = Msg;
    type Properties = Props;

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
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

    fn destroy(&mut self,_ctx: &Context<Self>) {
        self.on_destroy.clean();
    }

    fn view(&self,ctx: &Context<Self>) -> Html {
        match &self.state {
            State::Loading => loading(),
            State::Fetched(room,your_id) => match room.phase {
                Phase::Meeting =>  if !room.is_host {
                    html! {<Meeting room_id={self.props.room_id.clone()} on_error={self.props.on_error.clone()}/>}
                } else {
                    let start = ctx.link().callback(|_| Msg::Start);
                    html! {<MeetingHost room_id={self.props.room_id.clone()} start={start} on_error={self.props.on_error.clone()}/>}
                },
                Phase::Started => match your_id {
                    Some(your_id) => html! {
                        <Main 
                            is_host={room.is_host}
                            room_id={self.props.room_id.clone()}
                            your_id={your_id.clone()}
                            on_error={self.props.on_error.clone()}
                        />
                    },
                    None => not_found(),
                }
                
                
            }
            State::NotExists => not_found(),
        }
    }
    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props().clone();
        let room_id = ctx.props().room_id.as_str();
        let callback = ctx.link().callback(
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
            on_destroy
        }
    }
}