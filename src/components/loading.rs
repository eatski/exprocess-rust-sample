use yew::{html, virtual_dom::VNode};

pub fn loading() -> VNode{
    html! {
        <img src="/assets/loading.svg"/>
    }
}