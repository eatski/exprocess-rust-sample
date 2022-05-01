use yew::prelude::*;

use crate::util::get_value_from_event;
pub struct OpenForm {
    value: String,
    props: Props,
    first_focus: bool,
}

type OnSubmit = Callback<String>;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub on_submit: OnSubmit,
}

pub enum Msg {
    Change(String),
    Submit,
    Focus
}

impl Component for OpenForm {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            value: "ホスト".into(),
            props: ctx.props().clone(),
            first_focus: false
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Change(value) => self.value = value,
            Msg::Submit => {
                self.props.on_submit.emit(self.value.clone());
            }
            Msg::Focus => {
                if !self.first_focus {
                    self.first_focus = true;
                    self.value = "".into();
                }
            },
        }
        true
    }

    fn view(&self,ctx: &Context<Self>) -> Html {
        let onchange = ctx.link().callback(Msg::Change).reform(get_value_from_event);
        let onclick = ctx.link().callback(|_| Msg::Submit);
        let value = self.value.clone();
        html! {
            <div class="columns">
                <div class="column is-one-fifth">
                    <input type="text" class="input" placeholder="あなたの名前" {value} minlength=1 {onchange} onfocus={ctx.link().callback(|_| Msg::Focus)}/>
                </div>
                <div class="column">
                    <button class="button is-link" {onclick}>{"部屋を作成"}</button>
                </div>
            </div>
        }
    }
}
