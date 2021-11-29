use yew::{html, Html};

pub fn title() -> Html {
   html! {
        <>
            <h1 class="title is-2">{"Roll Role"}</h1>
            <h2 class="subtitle is-5">{"役職振り分けツール"}</h2>
        </>
    }
}

pub fn title_description() -> Html {
    html! {
        <p>{"複数人に対してランダムかつ本人にしか見えないように役職やワードを振り分けることができるツールです。"}</p>
    }
}


