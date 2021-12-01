use cafeteria::yew::{GalleryConfig, GalleryModel, Gallery, picture};
use presentation::{
    home::home,
    meeting::{meeting_guest, GuestForm},
    members::Member,
    sleep::sleep,
};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub struct Config;

impl GalleryConfig for Config {
    fn model() -> GalleryModel {
        GalleryModel::new([
            ("home",picture(|| home(&Callback::noop()))),
            ("meeting",picture(|| meeting_guest(
                &GuestForm::Joinable {
                    join: Callback::noop(),
                },
                &vec![
                    Member {
                        name: "aaaa".to_string(),
                        you: true,
                    },
                    Member {
                        name: "iii".to_string(),
                        you: false,
                    },
                ],
            ))),
            ("sleep",picture(sleep))
        ])
    }
}

pub fn main() {
    panic!()
}

#[wasm_bindgen(start)]
pub fn start() {
    yew::start_app::<Gallery<Config>>();
}
