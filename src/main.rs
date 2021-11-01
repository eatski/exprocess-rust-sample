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

pub enum Msg {
    Sleep,
    ReBoot
}

pub struct App {
    sleep: bool,
    link: ComponentLink<Self>
}
impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            sleep: false,
            link
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Sleep => {
                self.sleep = true;
                true
            },
            Msg::ReBoot => {
                self.sleep = false;
                self.set_timer();
                true
            },
        }
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
                    {if self.sleep {
                        html! {
                            <main>
                                <h2>{"Are You Sleeping?"}</h2>
                                <button onclick={self.link.callback(|_| Msg::ReBoot)}>{"Restart"}</button>
                            </main>
                            
                        }
                    } else {
                        html! {
                            <AppRouter
                                render=AppRouter::render(Self::switch)
                                redirect=AppRouter::redirect(|_| panic!())
                            />
                        }
                    }}
                    
                </main>
            </>
        }
    }
}
impl App {
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
