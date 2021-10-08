use yew::prelude::*;
pub struct Input {
    value: String,
    on_submit: OnSubmit,
    link: ComponentLink<Self>
}

type OnSubmit =  Callback<String>;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub on_submit: OnSubmit,
}

pub enum Msg {
    Change(String),
    Reset
}

impl Component for Input {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            on_submit: props.on_submit,
            value: String::from(""),
            link
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Change(value) => self.value = value,
            Msg::Reset => self.value = String::from("")
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let link = self.link.clone();
        let onchange = Callback::from(move |e| {
            match e {
                ChangeData::Value(value) => {
                    link.send_message(Msg::Change(value));
                },
                _ => ()
            }
        });
        let on_submit = self.on_submit.clone();
        let value = self.value.clone();
        let link = self.link.clone();
        let onclick= Callback::once(move |_| {
            on_submit.emit(value);
            link.send_message(Msg::Reset);
        });
        let value = self.value.clone();
        html! {
            <div>
                <input value=value onchange=onchange/>
                <button onclick=onclick>{"Join"}</button>
            </div>
        }
    }

}