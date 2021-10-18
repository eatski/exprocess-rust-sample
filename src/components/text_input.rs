use yew::prelude::*;
pub struct Input {
    value: String,
    link: ComponentLink<Self>,
    props: Props
}

type OnSubmit =  Callback<String>;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub on_submit: OnSubmit,
    pub button: String,
}

pub enum Msg {
    Change(String),
    Submit
}

impl Component for Input {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props: props,
            value: String::from(""),
            link
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Change(value) => self.value = value,
            Msg::Submit => {
                self.props.on_submit.emit(self.value.clone());
                self.value = String::from("");
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let onchange = self.link.callback(|data|
            match data {
                ChangeData::Value(value) => Msg::Change(value),
                _ => panic!("Invalid Type")
            }
        );
        let onclick= self.link.callback_once(|_| Msg::Submit);
        let value = self.value.clone();
        html! {
            <div>
                <input value=value onchange=onchange/>
                <button onclick=onclick>{&self.props.button}</button>
            </div>
        }
    }

}