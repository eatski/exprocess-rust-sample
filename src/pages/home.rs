use presentation::{home::home};
use yew::prelude::*;
use yew_router::history::{AnyHistory, History};
use yew_router::{prelude::*};
use crate::{routing::AppRoute};
use js_bridge::{create_room};

pub struct Home {
    state: State,
}

pub enum State {
    Init {
        on_submit: Callback<String>
    },
}

pub enum Msg {
    CreateRoom(String)
}

#[derive(Properties,Clone,PartialEq)]
pub struct Props {
    pub on_error: Callback<()>
}
impl Component for Home {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            state: State::Init {
                on_submit: ctx.link().callback(Msg::CreateRoom)
            }
        }
    }

    fn update(&mut self, ctx: &Context<Self>,msg: Self::Message) -> bool {
        match msg {
            Msg::CreateRoom(name) => {
                let on_error = ctx.props().on_error.clone();
                let id = create_room(
                    &name,
                    || {},
                    move || on_error.clone().emit(())
                );
                let route = AppRoute::Room { id };
                let history = AnyHistory::Browser(BrowserHistory::new());
                history.push(route);
            },
        }
        true
    }

    fn view(&self,_ctx: &Context<Self>) -> Html {
        match &self.state {
            State::Init { on_submit } => home(on_submit),
        }
        
    }
}
