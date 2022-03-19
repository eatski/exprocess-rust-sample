use cafeteria::yew::{dir, picture, Gallery, GalleryConfig, GalleryModel};
use presentation::{
    home::home,
    meeting::{meeting_guest, GuestForm},
    members::Member,
    sleep::sleep,
    set_role::HostForm, rolled,
};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub struct Config;

impl GalleryConfig for Config {
    fn model() -> GalleryModel {
        GalleryModel::new([
            ("home", picture(|| home(&Callback::noop()))),
            (
                "meeting",
                dir([
                    (
                        "guest",
                        picture(|| {
                            meeting_guest(
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
                            )
                        }),
                    ),
                    ("host", picture(|| todo!())),
                ]),
            ),
            ("role", dir([
                ("form",picture(|| html!{<HostForm on_submit=Callback::noop() members_num=6/>})),
                ("picked", picture(|| rolled::rolled("太郎","人狼", Some(Callback::noop())))),
            ])),
            ("sleep", picture(sleep)),
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
