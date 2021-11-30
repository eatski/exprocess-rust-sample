use presentation::{
    home::home,
    meeting::{meeting_guest, GuestForm},
    members::Member,
    sleep::sleep,
};
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Debug, Switch)]
pub enum AppRoute {
    #[to = "/!"]
    Home,
    #[to = "/meeting"]
    Meeting,
    #[to = "/sleep"]
    Sleep
}

pub type AppRouter = Router<AppRoute>;
pub struct Showcase;

impl Component for Showcase {
    type Message = ();

    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        todo!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        todo!()
    }

    fn view(&self) -> Html {
        let render = AppRouter::render(move |switch: AppRoute| match switch {
            AppRoute::Home => home(&Callback::noop()),
            AppRoute::Meeting => meeting_guest(
                &GuestForm::Joinable {
                    join: Callback::noop(),
                },
                &vec![
                    Member {
                        name: "aaaa".to_string(),
                        you: true,
                    },
                    Member {
                        name: "iii".to_string(),
                        you: false,
                    },
                ],
            ),
            AppRoute::Sleep => sleep(),
        });
        html! {
            <AppRouter
                render=render
                redirect=AppRouter::redirect(|_| panic!())
            />
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
