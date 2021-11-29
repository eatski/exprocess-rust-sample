use yew::prelude::*;

use crate::how_to_use::how_to_use;
use crate::loading::loading;
use crate::members::{Member, members_view};
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
                <button onclick=onclick class="button is-link">{"はじめる"}</button>
            </div>
            <div class="column">
                {how_to_use()}
            </div>
        </div>
    }
}




