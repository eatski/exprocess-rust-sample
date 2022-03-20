use web_sys::{HtmlInputElement};
use yew::{prelude::*};

pub fn get_value_from_event(event: Event) -> String {
    let input: HtmlInputElement = event.target_unchecked_into();
    input.value()
}

pub fn get_value_from_input(event: InputEvent) -> String {
    let input: HtmlInputElement = event.target_unchecked_into();
    input.value()
}