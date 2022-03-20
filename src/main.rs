use presentation::layout::layout;
use wasm_bindgen::prelude::*;

use yew::prelude::*;
use yew_router::prelude::*;

mod containers;
mod domain;
mod pages;
mod routing;

use pages::{home::Home, room::Room};

use presentation::{error};

use crate::routing::{AppRoute};

pub enum Msg {
    Error,
}

pub enum State {
    Error,
    Ok,
}

pub struct App {
    state: State
}
impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            state: State::Ok,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>,msg: Self::Message) -> bool {
        match msg {
            Msg::Error => {
                self.state = State::Error;
                true
            }
        }
    }

    fn view(&self,ctx: &Context<Self>) -> Html {
        layout(match self.state {
            State::Error => error::error(),
            State::Ok => {
                let on_error = ctx.link().clone().callback(|_| Msg::Error);
                html! {
                    <BrowserRouter>
                        <Switch<AppRoute> render={Switch::render(move |routes| switch(routes,on_error.clone()))} />
                    </BrowserRouter>
                }
            }
        })
    }
}

fn switch(routes: &AppRoute, on_error: Callback<()>) -> Html {
    match routes {
        AppRoute::Home => html! { <Home {on_error}/> },
        AppRoute::Room { id } => html! { <Room room_id={id.clone()} {on_error}/> },
    }
}

#[wasm_bindgen]
pub fn start(mode: AppMode) {
    let log_level = match mode {
        AppMode::Dev => log::Level::Trace,
        AppMode::Production => log::Level::Error,
    };
    wasm_logger::init(wasm_logger::Config::new(log_level));
    yew::start_app::<App>();
}

// コンパイルエラー回避のため仕方なく
pub fn main() {
    panic!()
}

#[wasm_bindgen]
pub enum AppMode {
    Dev,
    Production,
}
