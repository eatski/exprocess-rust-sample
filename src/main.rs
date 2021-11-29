use presentation::layout::layout;
use wasm_bindgen::prelude::*;
use webutil::util::set_timeout_no_mousemove;
use yew::prelude::*;

mod containers;
mod domain;
mod pages;
mod repository;

use pages::{home::Home, room::Room};
mod switch;
use switch::{AppRoute, AppRouter, PublicUrlSwitch};

use presentation::error;

pub enum Msg {
    Sleep,
    ReBoot,
    Error,
}

pub enum State {
    Sleep,
    Error,
    Ok,
}

pub struct App {
    state: State,
    link: ComponentLink<Self>,
    cleanable: Option<Box<dyn FnOnce()>>
}
impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            state: State::Ok,
            link,
            cleanable: None
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match (msg, &self.state) {
            (Msg::Sleep, State::Ok) => {
                self.state = State::Sleep;
                true
            }
            (Msg::ReBoot, State::Sleep) => {
                self.state = State::Ok;
                self.set_timer();
                true
            }
            (_, State::Error) => false,
            (Msg::Sleep, State::Sleep) => false,
            (Msg::ReBoot, State::Ok) => false,
            (Msg::Error, _) => {
                self.state = State::Error;
                true
            }
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.set_timer();
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn destroy(&mut self) {
        self.cleanable.take().map(|c| c());
    }

    fn view(&self) -> Html {
        layout(
            match self.state {
                State::Sleep => html! {
                    <div>
                        <h2>{"Are You Sleeping?"}</h2>
                        <button onclick={self.link.callback(|_| Msg::ReBoot)}>{"Restart"}</button>
                    </div>
                },
                State::Error => error::error(),
                State::Ok => {
                    let link = self.link.clone();
                    let render = AppRouter::render(move |switch: PublicUrlSwitch| {
                        let on_error = link.callback(|_| Msg::Error);
                        match switch.route() {
                            AppRoute::Home => {
                                html! { <Home on_error=on_error/> }
                            }
                            AppRoute::Room(room_id) => {
                                html! { <Room room_id=room_id on_error=on_error/> }
                            }
                        }
                    }); 
                    html! {
                        <AppRouter
                            render=render
                            redirect=AppRouter::redirect(|_| panic!())
                        />
                    }
                }
            }
        )
    }
}
impl App {
    fn set_timer(&mut self) {
        let sleep = self.link.callback(|_| Msg::Sleep);
        self.cleanable = Some(set_timeout_no_mousemove(
            move || {
                sleep.emit(());
            },
            1000 * 60 * 30,
            1000,
        ));
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
