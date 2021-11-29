use yew::{Html, html};

pub fn main_vis_container(content: Html) -> Html {
    html! {
        <div class="hero is-info">
            <div class="hero-body">
                {content}
            </div>
        </div>
    }
}