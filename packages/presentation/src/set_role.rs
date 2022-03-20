use mytil::validate_no_duplicate;

use yew::{prelude::*};
use crate::{how_to_use::how_to_use, members::{Member, members_view}, util::{get_value_from_event, get_value_from_input}};

pub type FormInputs = Vec<RoleInput>;

pub fn set_role_guest(members: &Vec<Member>) -> Html {
    html! {
        <section class="section">
            <h3 class="title is-4">{"ホストが役職を決定しています。"}</h3>
            {members_view(members)}
        </section>
    }
}

pub fn set_role_host(members: &Vec<Member>,on_submit: &Callback<FormInputs>) -> Html {
    html! {
        <section class="section columns">
            <div class="column">
                <h3 class="title is-4">{"役職を入力しましょう。"}</h3>
                {members_view(members)}
                <HostForm {on_submit} members_num={members.len()}/>
            </div>
            <div class="column">
                {how_to_use()}
            </div>
        </section>
    }
}

pub struct HostForm {
    pub props: Props,
    pub inputs: Vec<RoleInput>,
}

#[derive(Clone)]
pub struct RoleInput {
    pub name: String,
    pub num: usize
}

#[derive(Clone,PartialEq,Properties)]
pub struct Props {
    pub on_submit: Callback<FormInputs>,
    pub members_num: usize
}

pub enum Msg {
    UpdateNum {
        index: usize,
        num: usize
    },
    UpdateName {
        index: usize,
        name: String
    },
    AddInput 
}

impl Component for HostForm {
    type Message = Msg;

    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props().clone();
        Self {
            inputs: vec![
                RoleInput {name:String::from("Hero"),num:1},
                RoleInput {name:String::from("Mob"),num:props.members_num - 1}
            ],
            props,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>,msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateNum { index, num } => {
                let mut input = self
                    .inputs
                    .get_mut(index)
                    .unwrap();
                input.num = num;
            },
            Msg::UpdateName { index, name } => {
                let mut input = self
                    .inputs
                    .get_mut(index)
                    .unwrap();
                input.name = name;
            },
            Msg::AddInput => {
                self.inputs.push(RoleInput {name:String::from(""),num:1})
            },
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link().clone();
        let len = self.inputs.len();
        let inputs = self
            .inputs
            .iter()
            .enumerate()
            .map(|(index,role_input)| role_input_view(
                role_input,
                link.callback(|e| e),index,
                len - 1 == index,
                index == 0
            ));
           
        let valid = validate(&self.inputs, self.props.members_num);
        let on_submit = valid.then(|| {
            let on_submit = self.props.on_submit.clone();
            let inputs_capture = self.inputs.clone();
            Callback::once(move |_| on_submit.emit(inputs_capture))
        });
        html! {
            <div>
                <ul class="field">
                    {for inputs}
                </ul>
                <div class="field columns">
                    <div class="column is-offset-8"> 
                    { 
                        html! {
                            <button class="button is-link" disabled={!valid} onclick={on_submit}>{"決定"}</button>
                        }
                    }
                    </div>
                </div>
                {
                    if valid {
                        html!{}
                    } else {
                        html!{<div class="notification is-danger is-light">{"入力が不正です。"}</div>}
                    }
                }
            </div>
        }
    }
}

fn role_input_view(input:&RoleInput,callback: Callback<Msg>,index: usize,is_last:bool,is_first: bool) -> Html {
    let num = input.num.to_string();
    let name = input.name.clone();
    let on_num_change = callback.reform(move |event| {
        let num = get_value_from_event(event);
        Msg::UpdateNum {
            index,
            num: num.parse().expect("input value is not number")
        }
    });
    let on_name_change = callback.reform(move |event: InputEvent| {
        let name = get_value_from_input(event);
        Msg::UpdateName {
            name,
            index
        }
    });
    let label_class = if is_first {"label"} else {"label is-hidden-tablet"};
    html! {
        <li class="is-grouped columns">
            <div class="control column">
                <label class={label_class}>{"役職名"}</label>
                <input class="input" type="text" value={name} oninput={on_name_change} />
            </div>
            <div class="control column is-2">
                <label class={label_class}>{"数"}</label>
                <input class="input" type="number" value={num} min={1} onchange={on_num_change}/>
            </div>
            <div class="control column is-2 is-flex is-align-items-center">
            {
                is_last.then(|| {
                    let add = callback.reform(|_| Msg::AddInput);
                    html!{
                        <div class="icon is-medium is-clickable" onclick={add}>
                            <i class="fas fa-plus-circle"></i>
                        </div>
                    }
                }).unwrap_or_default()
            }
            </div>
            
        </li>
    }
}

fn validate(inputs: &Vec<RoleInput>,members_num: usize) -> bool {
    if inputs.iter().map(|input| input.num).sum::<usize>() < members_num {
        return false;
    } else if !validate_no_duplicate(inputs.iter().map(|role| role.name.as_str())) {
        return false;
    } {
        return inputs.iter().all(|role| !role.name.is_empty())  
    }
}

