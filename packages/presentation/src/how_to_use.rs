use yew::prelude::*;

pub fn how_to_use() -> Html {
    html! {
        <div class="content">
            <h3>{"使い方"}</h3>
            <ul>
                <li>{"このページのURLを他の参加者に共有してください。"}</li>
                <li>{"参加者が集まるのを待ちます。"}</li>
                <li>{"集まったら「はじめる」を押しましょう。"}</li>
                <li>{"役職名とそれが振り分けられる人数を入力し、「Roll」を押しましょう。"}</li>
                <li>{"それぞれの画面に自分の役職が表示されます。"}</li>
            </ul>
        </div>
    }
}