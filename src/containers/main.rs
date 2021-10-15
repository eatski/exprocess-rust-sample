use yew::prelude::*;
use crate::domain::{Runner, exprocess::AppState, exprocess::AppCommand};

pub struct Main {
    runner:Runner
}

pub struct ViewState {

}

fn app_state_to_view_state(app:&AppState) -> ViewState {
    todo!()
}

pub enum Msg {
    UpdateState(ViewState)
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
            Box::new(move |_,state| link.send_message(Msg::UpdateState(app_state_to_view_state(state))))
        );
        Main {
            runner
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.runner.dispatch(AppCommand::Init);
        todo!();
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