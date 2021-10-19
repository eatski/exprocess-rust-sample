use std::{ops::Add, vec};

use yew::{prelude::*, virtual_dom::VNode};

use crate::domain::exprocess::{PickCommand,Role};

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
            Msg::AddInput => todo!(),
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
        let valid = (self.inputs.iter().map(|input| input.num).fold(0, Add::add )) == self.props.members_num;
        html! {
            <div>
                <ul>
                    {for inputs}
                </ul>
                { if valid {
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
                } else {html! {}} }
            </div>
        }
    }
}

fn role_input_view(input:&RoleInput,link: &ComponentLink<HostForm>,index: usize) -> VNode {
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
    let on_name_change = link.callback(move |change| {
        match change {
            ChangeData::Value(name) => Msg::UpdateName {
                index,
                name
            },
            _ => panic!()
        }
    });
    html! {
        <li>
            <input type="text" value=name onchange=on_name_change />
            <input type="number" value=num min=1 onchange=on_num_change/>
        </li>
    }
}