use yew::prelude::*;
use crate::domain::{Runner, exprocess::AppState,exprocess::AppCommand};

pub struct Main {
    runner:Runner,
    state: ViewState,
    props: Props
}

pub enum ViewState {
    Blank,
    Standby(Vec<String>)
}

fn app_state_to_view_state(app:&AppState) -> ViewState {
    match app {
        AppState::Blank => ViewState::Blank,
        AppState::Standby(members) => ViewState::Standby(members.clone()),
        AppState::Picked => todo!(),
    }
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
            state: ViewState::Blank,
            runner,
            props
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateState(state) => {
                self.state = state;
            },
        };
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        panic!()
    }

    fn rendered(&mut self, _first_render: bool) {
        if _first_render && self.props.is_host {
            self.runner.dispatch(AppCommand::Init(vec![]))
        }
    }

    fn view(&self) -> Html {
        match &self.state {
            ViewState::Blank => html! {
                "Started"
            },
            ViewState::Standby(members) => {
                html! {
                    <ul>
                        {for members.iter().map(|member| {
                            html! {
                                <li>{member}</li>
                            }
                        })}
                    </ul>
                }
            },
        }
        
    }
}