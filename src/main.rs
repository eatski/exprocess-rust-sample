use webutil::util::set_timeout_no_mousemove;
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

use crate::components::loading::loading;

pub enum Msg {
    Sleep,
    ReBoot,
    Load,
    StopLoading
}

pub struct App {
    state: State,
    link: ComponentLink<Self>
}

pub enum GlobalMessage {
    Load,
    StopLoading
}

pub enum State {
    Sleep,Loading,Active
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {

        Self {
            state: State::Active,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Sleep => {
                self.state = State::Sleep;
                
            },
            Msg::ReBoot => {
                if matches!(self.state,State::Sleep) {
                    self.state = State::Active;
                };
                self.set_timer();
            },
            Msg::Load => {
                self.state = State::Loading;
            },
            Msg::StopLoading => {
                if matches!(self.state,State::Loading) {
                    self.state = State::Active;
                };
            },
        };
        true
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render { self.set_timer();}
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <header>
                    <a href="/">
                        <img src="/assets/favicon.ico" />
                        <strong>{"Roll Role"}</strong>
                    </a>
                    
                </header>
                <main>
                    {match self.state {
                        State::Sleep => {
                            html! {
                                <main>
                                    <h2>{"Are You Sleeping? ///...zzz "}</h2>
                                    <button onclick={self.link.callback(|_| Msg::ReBoot)}>{"Restart"}</button>
                                </main>
                            }
                        },
                        State::Loading => loading(),
                        State::Active => {
                            let dispatch_global = self.link.callback(|loading| {
                                match loading {
                                    GlobalMessage::Load => Msg::Load,
                                    GlobalMessage::StopLoading => Msg::StopLoading,
                                }
                            });
                            html! {
                                <AppRouter
                                    render=AppRouter::render(move |switch| Self::switch(switch, dispatch_global.clone()))
                                    redirect=AppRouter::redirect(|_| panic!())
                                />
                            }
                        },
                    }}
                </main>
            </>
        }
    }
}
impl App {
    fn switch(switch: PublicUrlSwitch,dispatch_global: Callback<GlobalMessage> ) -> Html {
        match switch.route() {
            AppRoute::Home => {
                html! { <Home dispatch_global=dispatch_global /> }
            }
            AppRoute::Room(room_id) => {
                html! { <Room room_id=room_id dispatch_global=dispatch_global/> }
            }
        }
    }
    fn set_timer(&mut self) {
        let sleep = self.link.callback(|_| Msg::Sleep);
        drop(set_timeout_no_mousemove(move || {sleep.emit(());}, 1000 * 60 * 30, 1000));
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
pub enum AppMode{
    Dev,Production
}
