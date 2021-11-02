use yew::prelude::*;
use yew_router::{agent::RouteRequest, prelude::*};
use crate::GlobalMessage;
use crate::repository::{create_room};
use crate::components::text_input::{Input};

use crate::switch::AppRoute;

pub struct Home {
    state: State,
    dispatch_global: Callback<GlobalMessage>
}

pub enum State {
    Init {
        on_submit: Callback<String>
    },
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub dispatch_global: Callback<GlobalMessage>
}

pub enum Msg {
    CreateRoom(String)
}
impl Component for Home {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            state: State::Init {
                on_submit:  link.callback(Msg::CreateRoom)
            },
            dispatch_global: props.dispatch_global
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::CreateRoom(name) => {
                self.dispatch_global.emit(GlobalMessage::Load);
                let dispatch_global = self.dispatch_global.clone();
                create_room(
                    &name,
                    Box::new(move |id| {
                        dispatch_global.emit(GlobalMessage::StopLoading);
                        let route = AppRoute::Room(id);
                        let mut dispatcher = RouteAgentDispatcher::new();
                        dispatcher.send(RouteRequest::ChangeRoute(route.into_route()));
                    })
                );
                
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
                        <h2>{ "Roll Role" }</h2>
                        <p>{"This app determines your role at random by rolling. "}</p>
                        <Input on_submit=on_submit value="host" button="Join"/>
                    </div>
                }
            },
        }
        
    }
}
