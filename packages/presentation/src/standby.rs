use yew::prelude::*;

use crate::roles::{Role, roles_table};

pub fn standby(roles: &Vec<(usize,Role)>, start: &Callback<()>) -> Html {
    html! {
        <section class="section">
            <h3 class="title is-4">{"準備OK!!"}</h3>
            {roles_table(roles)}
            <button onclick=start.reform(|_| ()) class="button is-link">{"Roll"}</button>
        </section>
    }
}

pub fn standby_guest(roles: &Vec<(usize,Role)>) -> Html {
    html! {
        <section class="section">
            <h3 class="title is-4">{"待機中..."}</h3>
            {roles_table(roles)}
        </section>
    }
}