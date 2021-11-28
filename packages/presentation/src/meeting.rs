use yew::prelude::*;

use crate::title::title;
pub enum Props {
    Joinable {
        join: Callback<String> 
    },
    Joined,
    Loading
}

pub fn meeting_guest(state: Props) -> Html {
    html! {
        <>
            {title()}
        </>
    }
}