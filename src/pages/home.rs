use yew::prelude::*;
use yew_router::{agent::RouteRequest, prelude::*};
use crate::components::loading;
use crate::repository::{create_room};
use crate::components::text_input::{Input};

use crate::switch::AppRoute;

pub struct Home {
    state: State
}

pub enum State {
    Init {
        on_submit: Callback<String>
    },
}

pub enum Msg {
    CreateRoom(String)
}
impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            state: State::Init {
                on_submit:  link.callback(Msg::CreateRoom)
            }
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::CreateRoom(name) => {
                let id = create_room(
                    &name,
                    Box::new(|| {}));
                let route = AppRoute::Room(id);
                let mut dispatcher = RouteAgentDispatcher::new();
                dispatcher.send(RouteRequest::ChangeRoute(route.into_route()));
            },
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        match &self.state {
            State::Init { on_submit } => {
                html! {
                    <div>
                        <h2>{ "Home" }</h2>
                        <Input on_submit=on_submit button="Join"/>
                    </div>
                }
            },
        }
        
    }
}
