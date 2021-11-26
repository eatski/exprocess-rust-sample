use std::{vec};

use mytil::validate_no_duplicate;
use yew::{prelude::*};

use crate::domain::state::{PickCommand,Role};

pub struct HostForm {
    pub props: Props,
    pub inputs: Vec<RoleInput>,
    pub link: ComponentLink<Self>
}

pub struct RoleInput {
    pub name: String,
    pub num: usize
}

#[derive(Clone,PartialEq,Properties)]
pub struct Props {
    pub on_submit: Callback<PickCommand>,
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

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            inputs: vec![
                RoleInput {name:String::from("Hero"),num:1},
                RoleInput {name:String::from("Mob"),num:props.members_num - 1}
            ],
            props,
            link
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
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

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        //FIXME: 何？
        false
    }

    fn view(&self) -> Html {
        let link = self.link.clone();
        let inputs = self
            .inputs
            .iter()
            .enumerate()
            .map(move |(index,role_input)| role_input_view(role_input,&link,index));
        let nums_gt_players = (self.inputs.iter().map(|input| input.num).sum::<usize>()) >= self.props.members_num;
        let names_no_duplicate = validate_no_duplicate(self.inputs.iter().map(|role| role.name.as_str()));
        let names_no_empty = self.inputs.iter().all(|role| !role.name.is_empty());
        let add_input = self.link.callback_once(|_| Msg::AddInput);
        html! {
            <div>
                <ul>
                    {for inputs}
                </ul>
                <div>
                    <button onclick=add_input>{"Add Role"}</button>
                </div>
                { 
                    if nums_gt_players && names_no_duplicate && names_no_empty {
                    let command = PickCommand {
                        roles: self.inputs.iter()
                            .map(|input| (input.num, Role {name: input.name.clone()}))
                            .collect()
                    };
                    let on_submit = self.props.on_submit.clone();
                    let onclick = Callback::once(move |_| on_submit.emit(command));
                    html! {
                        <button onclick=onclick>{"Roll!!"}</button>
                    }
                } else {
                    html! { }
                }
            }
            </div>
        }
    }
}

fn role_input_view(input:&RoleInput,link: &ComponentLink<HostForm>,index: usize) -> Html {
    let num = input.num.to_string();
    let name = input.name.clone();
    let on_num_change = link.callback(move |change| {
        match change {
            ChangeData::Value(value) => Msg::UpdateNum {
                index,
                num: value.parse().unwrap()
            },
            _ => panic!()
        }
    });
    let on_name_change = link.callback(move |input: InputData| {
        Msg::UpdateName {
            name:input.value,
            index
        }
    });
    html! {
        <li>
            <input type="text" value=name oninput=on_name_change />
            <input type="number" value=num min=1 onchange=on_num_change/>
        </li>
    }
}

