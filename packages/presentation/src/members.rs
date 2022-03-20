use yew::prelude::*;

pub struct Member {
    pub you: bool,
    pub name: String
}

pub fn members_view(members:&Vec<Member>)-> Html {
    html! {
        <table class="table">
            <thead>
                <tr>
                    <th>{"参加者"}</th>
                    <th></th>
                </tr>
            </thead>
            <tbody>
                {for members.iter().map(|member| html! {
                    <tr>
                        <td>
                            <span>{member.name.as_str()}</span>
                        </td>
                        <td>
                            {if member.you {html! {<span class="tag is-primary ml-2">{"YOU"}</span>}} else {html! {}}}
                        </td>
                    </tr>
                })}
            </tbody>
        </table>
    }
}