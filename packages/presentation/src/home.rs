use yew::prelude::*;

use crate::title::title;
use crate::open_form::OpenForm;
pub fn home(on_submit: &Callback<String>) -> Html {
    html! {
        <>
            {title()}
            <div class="section">
                <OpenForm on_submit=on_submit />
            </div>
        </>
    }
}