use yew::prelude::*;
use yew_router::{agent::RouteRequest, prelude::*};
use crate::repository::{create_room,MembersRepository};
use crate::components::input::{Input};

use crate::switch::AppRoute;

pub struct Home {
    on_submit: Callback<String>
}

pub enum Msg {
    CreateRoom(String)
}
impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            on_submit: link.callback(Msg::CreateRoom)
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::CreateRoom(name) => {
                create_room(Box::new(move |id| {
                    let id_clone = id.clone();
                    let route = AppRoute::Room(id);
                    let repo = MembersRepository::new(id_clone);
                    repo.save(name);
                    let mut dispatcher = RouteAgentDispatcher::new();
                    dispatcher.send(RouteRequest::ChangeRoute(route.into_route()));
                    
                }));
            },
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h2>{ "Home" }</h2>
                <Input on_submit=&self.on_submit/>
            </div>
        }
    }
}
