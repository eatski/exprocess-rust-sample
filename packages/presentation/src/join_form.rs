use yew::{prelude::*};

use crate::util::get_value_from_event;
pub struct JoinForm {
    value: String,
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

impl Component for JoinForm {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            value: "".into(),
            props: ctx.props().clone(),
        }
    }

    fn update(&mut self,  _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Change(value) => self.value = value,
            Msg::Submit => {
                self.props.on_submit.emit(self.value.clone());
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onchange = ctx.link().callback(Msg::Change).reform(get_value_from_event);
        let onclick = ctx.link().callback(|_| Msg::Submit);
        let value = self.value.clone();
        html! {
            <div class="columns">
                <div class="column is-one-fifth">
                    <input type="text" class="input" placeholder="あなたの名前" {value} minlength=1 {onchange}/>
                </div>
                <div class="column">
                    <button class="button is-link" {onclick}>{"参加する"}</button>
                </div>
            </div>
        }
    }
}
