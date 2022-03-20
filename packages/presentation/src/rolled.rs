use yew::{html, Html, Callback};

pub fn rolled(name: &str,role: &str, next: &Option<Callback<()>>) -> Html {
    html! {
        <>
        <section class="section">
            <h3 class="title is-4">{"あなたに役職が振られました。"}</h3>
            <p class="block">
                {"あなた（"}
                {html! {<strong class="has-text-info">{name}</strong>}}
                {"）は"}
                {html! {<strong class="has-text-info">{role}</strong>}}
                {"です。"}
            </p>
            {next.as_ref()
                .map(|next| next.reform(|_| ()))
                .map(|onclick| html! {<p class="block"><button {onclick} class="button is-primary">{"もう一度"}</button></p>})
                .unwrap_or_default()
            }           
        </section>
        
        </>
    }
}