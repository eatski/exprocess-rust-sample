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
            <section class="section">
                {guest_form(form)}
                {members_view(members)}
            </section>
           
        </>
    }
}

pub fn guest_form(form: &GuestForm) -> Html {
    match form {
        GuestForm::Joinable { join } => {
            html! {
                <JoinForm on_submit=join />
            }
        },
        GuestForm::Joined => html! {
            <h3 class="title is-4">{"ホストが開始するのを待っています。"}</h3>
        },
        GuestForm::Loading => loading(),
    }
}

pub fn meeting_host(members:&Vec<Member>,start: &Callback<()>) -> Html {
    let onclick = start.reform(|_| ());
    html! { 
        <div class="section columns">
            <div class="column">
                <h3 class="title is-4">{"参加者を集めましょう。"}</h3>
                {members_view(members)}
                <button onclick=onclick class="button is-link">{"Roll"}</button>
            </div>
            <div class="column">
                {how_to_use()}
            </div>
        </div>
    }
}

pub struct Member {
    pub you: bool,
    pub name: String
}

fn members_view(members:&Vec<Member>)-> Html {
    let members = members.iter().map(|member| html! {
        <tr>
            <td>
                <span>{member.name.as_str()}</span>
            </td>
            <td>
                {if member.you {html! {<span class="tag is-primary ml-2">{"YOU"}</span>}} else {html! {}}}
            </td>
        </tr>
    });
    html! {
        <table class="table">
            <thead>
                <tr>
                    <th>{"参加者"}</th>
                    <th></th>
                </tr>
            </thead>
            <tbody>
                {for members}
            </tbody>
        </table>
    }
}

fn how_to_use() -> Html {
    html! {
        <div class="content">
            <h3>{"使い方"}</h3>
            <ul>
                <li>{"このページのURLを他の参加者に共有してください。"}</li>
                <li>{"参加者が集まるのを待ちます。"}</li>
                <li>{"集まったら'Roll'を押しましょう。"}</li>
                <li>{"それぞれの画面に自分の役職が表示されます。"}</li>
            </ul>
        </div>
    }
}


