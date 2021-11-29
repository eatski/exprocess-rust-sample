use yew::{html, Html};
pub fn sleep() -> Html{
    html! {
        <div class="section">
            <p class="title is-5">{"一定時間操作がないのでスリープしています。"}<strong class="has-text-info">{"zzz"}</strong></p>
            <p>{"ブラウザをリロードしてください。"}</p>
        </div>
    }
}