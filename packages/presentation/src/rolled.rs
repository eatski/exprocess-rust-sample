use yew::{html, Html};

pub fn rolled(name: &str,role: &str) -> Html {
    html! {
        <section class="section">
            // いい感じに書きたい
            <h3 class="title is-4">
                {"あなた（"}
                {html! {<strong class="has-text-info">{name}</strong>}}
                {"）は"}
                {html! {<strong class="has-text-info">{role}</strong>}}
                {"です。"}
            </h3>
        </section>
    }
}