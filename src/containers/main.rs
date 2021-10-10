use yew::prelude::*;

pub struct Main {

}


#[derive(Clone,Eq,PartialEq,Properties)]
pub struct Props {
    pub is_host: bool
}

impl Component for Main {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Main {}
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