use manifest::manifest;
use yew::{html, virtual_dom::VNode};

#[manifest("/unko")]
enum Names {
    Container,Content
}

pub fn not_found() -> VNode {
    html! {
        <section class=Names::Container>
            <div class=Names::Content>{"Not Found"}</div>
        </section>
    }
}