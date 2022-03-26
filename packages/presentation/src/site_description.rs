use yew::prelude::*;

pub fn site_description() -> Html {
    html! {
        <div class="content">
            <h3>{"このツールについて"}</h3>
            <ul>
                <li>{"結果が本人にしか見えないので自分の"}<strong>{"正体を隠匿"}</strong>{"しながら遊べる！"}</li>
                <li>{"ツールが役職の振り分けをやるので"}<strong>{"ゲームマスター不要"}</strong>{"!"}</li>
                <li>{"役職は自由に設定できるので"}<strong>{"工夫次第"}</strong>{"で好きに遊べる！"}</li>
            </ul>
        </div>
    }
}