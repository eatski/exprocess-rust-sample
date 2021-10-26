use yew::prelude::*;
use wasm_bindgen::prelude::*;

mod repository;
mod components;
mod containers;
mod domain;
mod pages;
use pages::{
    home::Home,room::Room,
};
mod switch;
use switch::{AppRoute, AppRouter, PublicUrlSwitch};
use yew_router::prelude::Route;

pub enum Msg {
    ToggleNavbar,
}

pub struct Model {
    navbar_active: bool,
}
impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            navbar_active: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <main>
                <AppRouter
                    render=AppRouter::render(Self::switch)
                    redirect=AppRouter::redirect(|_| panic!())
                />
            </main>
        }
    }
}
impl Model {
    fn switch(switch: PublicUrlSwitch) -> Html {
        match switch.route() {
            AppRoute::Home => {
                html! { <Home /> }
            }
            AppRoute::Room(room_id) => {
                html! { <Room room_id=room_id/> }
            }
        }
    }
}

#[wasm_bindgen]
pub fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::start_app::<Model>();
}
