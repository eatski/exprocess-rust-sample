use yew::{prelude::*};

use crate::util::get_value_from_event;
pub struct JoinForm {
    value: String,
}

type OnSubmit = Callback<String>;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub on_submit: OnSubmit,
}

pub enum Msg {
    Change(String)
}

impl Component for JoinForm {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: "".into(),
        }
    }

    fn update(&mut self,  _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Change(value) => self.value = value
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onchange = ctx.link().callback(Msg::Change).reform(get_value_from_event);
        let value = self.value.clone();
        let onclick = ctx.props().on_submit.reform(move |_| value.clone());
        html! {
            <div class="columns">
                <div class="column is-one-fifth">
                    <input type="text" class="input" placeholder="あなたの名前" value={self.value.clone()} minlength=1 {onchange}/>
                </div>
                <div class="column">
                    <button class="button is-link" {onclick}>{"参加する"}</button>
                </div>
            </div>
        }
    }
}
