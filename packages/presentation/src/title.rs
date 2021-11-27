use yew::{Html, html};

pub fn title() -> Html {
    html! {
        <section class="section">
            <h1 class="title">{"Roll Role"}</h1>
            <h2 class="subtitle">{"役職振り分けツール"}</h2>
            <p class="box">
                {"それぞれの人にランダムに本人にしか見えないように役職やワードを振り分けることができるツールです。"}
            </p>
        </section>
    }
}