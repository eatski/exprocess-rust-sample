use yew::prelude::*;

use crate::main_vis::main_vis_container;
use crate::site_description::site_description;
use crate::title::{title, title_description};
use crate::open_form::OpenForm;
pub fn home(on_submit: &Callback<String>) -> Html {
    html! {
        <>
            {main_vis_container(
                html! {
                    <>
                        {title()}
                        {title_description()}
                    </>
                }
            )}
            <div class="section">
                <OpenForm on_submit={on_submit} />
            </div>
            <section class="section">
                {site_description()}
            </section>
            
        </>
    }
}