use yew::{Html, html};

pub fn title() -> Html {
    html! {
        <div class="hero is-info">
            <div class="hero-body">
                <h1 class="title is-2">{"Roll Role"}</h1>
                <h2 class="subtitle is-5">{"役職振り分けツール"}</h2>
                <p>
                    {"複数人に対してランダムかつ本人にしか見えないように役職やワードを振り分けることができるツールです。"}
                </p>
            </div>
        </div>
    }
}