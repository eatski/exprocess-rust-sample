use yew::prelude::*;

use crate::loading::loading;
use crate::{main_vis::main_vis_container, title::title};
use crate::join_form::JoinForm;

pub enum GuestForm {
    Joinable { join: Callback<String> },
    Joined,
    Loading,
}

pub fn meeting_guest(form: &GuestForm,members:&Vec<Member>) -> Html {
    html! {
        <>
            {main_vis_container(title())}
            <div class="section">
                {
                    match form {
                        GuestForm::Joinable { join } => {
                            html! {
                                <JoinForm on_submit=join />
                            }
                        },
                        GuestForm::Joined => html! {},
                        GuestForm::Loading => loading(),
                    }
                }
                {members_view(members)}
            </div>
        </>
    }
}

pub fn meeting_host(members:&Vec<Member>,start: Callback<()>) -> Html {
    let onclick = start.reform(|_| ());
    html! { 
        <>
            <section>
                <h3> {"使い方"} </h3>
                <ul>
                    <li>{"このページのURLを他の参加者に共有してください。"}</li>
                    <li>{"参加者が集まるのを待ちます。"}</li>
                    <li>{"集まったら'Roll!!'を押しましょう。"}</li>
                    <li>{"それぞれの画面に自分の役職が表示されます。"}</li>
                </ul>
            </section>
            {members_view(members)}
            <button onclick=onclick>{"Roll!!"}</button>
        </>
    }
}

pub struct Member {
    pub you: bool,
    pub name: String
}

fn members_view(members:&Vec<Member>)-> Html {
    let members = members.iter().map(|member| html! {
        <li>
            <span>{member.name.as_str()}</span>
            {if member.you {html! {<span>{"⇨YOU"}</span>}} else {html! {}}}
        </li>
    });
    html! {
        <ul>{for members}</ul>
    }
}



