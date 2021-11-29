use presentation::{home::home, meeting::{meeting_host}, members::Member, sleep::sleep};
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
        html! {
            <>
                <div>
                    {home(&Callback::noop())}   
                </div>
                <div>
                    {meeting_host(
                        &vec![
                            Member {
                                name: "aaaa".to_string(),
                                you: true
                            },
                            Member {
                                name: "iii".to_string(),
                                you: false
                            },
                        ],
                        &Callback::noop()
                    )}
                </div>
                <div>
                    {sleep()}
                </div>
                
            </>
        }
        
    }
}

pub fn main() {
    panic!()
}

#[wasm_bindgen(start)]
pub fn start() {
    yew::start_app::<Showcase>();
}
