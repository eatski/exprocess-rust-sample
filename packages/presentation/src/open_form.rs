use yew::prelude::*;
pub struct OpenForm {
    value: String,
    link: ComponentLink<Self>,
    props: Props,
}

type OnSubmit = Callback<String>;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub on_submit: OnSubmit,
}

pub enum Msg {
    Change(String),
    Submit,
}

impl Component for OpenForm {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            value: "ホスト".into(),
            props: props,
            link,
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
        let onchange = self.link.callback(|data| match data {
            ChangeData::Value(value) => Msg::Change(value),
            _ => panic!("Invalid Type"),
        });
        let onclick = self.link.callback(|_| Msg::Submit);
        let value = self.value.clone();
        html! {
            <div class="columns">
                <div class="column is-one-fifth">
                    <input type="text" class="input" placeholder="あなたの名前" value=value minlength=1 onchange=onchange/>
                </div>
                <div class="column">
                    <button class="button is-link" onclick=onclick>{"はじめる"}</button>
                </div>
            </div>
        }
    }
}
