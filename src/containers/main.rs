use yew::prelude::*;
use crate::domain::{Runner, exprocess::AppState};

pub struct Main {
    runner:Runner
}

pub enum Msg {
    UpdateState(AppState)
} 

#[derive(Clone,Eq,PartialEq,Properties)]
pub struct Props {
    pub is_host: bool
}

impl Component for Main {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let runner = Runner::start(
            Box::new(move |_,state| link.send_message(Msg::UpdateState(state)))
        );
        Main {
            runner
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        todo!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        panic!()
    }

    fn view(&self) -> Html {
        html! {
            "Started"
        }
    }
}