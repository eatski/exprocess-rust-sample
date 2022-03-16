use yew::{html, Html};

pub struct Role {
    pub name: String,
}

pub fn roles_table(roles: &Vec<(usize,Role)>) -> Html {
    let roles = roles.iter().map(|(num,role)| html! {
        <tr>
            <td>
                <span>{role.name.as_str()}</span>
            </td>
            <td>
                {num}
            </td>
        </tr>
    });
    html! {
        <table class="table">
            <thead>
                <tr>
                    <th>{"役職"}</th>
                    <th>{"人数"}</th>
                </tr>
            </thead>
            <tbody>
                {for roles}
            </tbody>
        </table>
    }
}
