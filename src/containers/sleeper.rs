use mytil::{Cleaner, FnOnceCleanable};
use presentation::sleep::sleep;
use webutil::util::set_timeout_no_mousemove;
use yew::{Children, Component, ComponentLink, Properties};

pub struct Sleeper {
    link: ComponentLink<Self>,
    props: Props,
    sleep: bool,
    cleaner: Cleaner<FnOnceCleanable>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub children: Children,
}

pub enum Msg {
    Sleep,
}

impl Component for Sleeper {
    type Message = Msg;

    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            sleep: false,
            cleaner: Cleaner::empty(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::Sleep => {
                self.sleep = true;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        todo!()
    }

    fn view(&self) -> yew::Html {
        if self.sleep {
            sleep()
        } else {
            self.props.children.iter().collect()
        }
    }

    fn rendered(&mut self, _first_render: bool) {
        if _first_render {
            let link = self.link.clone();
            self.cleaner = set_timeout_no_mousemove(
                move || {
                    link.send_message(Msg::Sleep);
                },
                1000 * 60 * 30,
                1000,
            );
        }
    }
}
