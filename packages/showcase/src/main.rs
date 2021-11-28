use presentation::title::title;
use yew::{ prelude::*};
use yew_router::{ prelude::*};
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, Switch)]
pub enum AppRoute {
    #[to = "/!"]
    Home,
    #[to = "/title"]
    Title,
}
pub struct Showcase;

impl Component for Showcase {
    type Message = ();

    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        todo!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        todo!()
    }

    fn view(&self) -> Html {
        title()
    }
}

pub fn main() {
    panic!()
}

#[wasm_bindgen]
pub fn start() {
    yew::start_app::<Showcase>();
}
