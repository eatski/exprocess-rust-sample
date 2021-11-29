use yew::{Html, html};

pub fn not_found() -> Html{
    html! {
        <div class="section">
            <p class="title is-3">{"Not Found"}</p>
        </div>
    }
}