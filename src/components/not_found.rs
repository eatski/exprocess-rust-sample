use yew::{html, virtual_dom::VNode};

pub fn not_found() -> VNode{
    html! {
        <div>{"Not Found"}</div>
    }
}