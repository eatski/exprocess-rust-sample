use mytil::{Cleaner, FnOnceCleanable};
use presentation::sleep::sleep;
use webutil::util::set_timeout_no_mousemove;
use yew::{Children, Component, Properties, Context};

pub struct Sleeper {
    props: Props,
    sleep: bool,
    cleaner: Cleaner<FnOnceCleanable>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
}

pub enum Msg {
    Sleep,
}

impl Component for Sleeper {
    type Message = Msg;

    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().clone(),
            sleep: false,
            cleaner: Cleaner::empty(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>,msg: Self::Message) -> bool {
        match msg {
            Msg::Sleep => {
                self.sleep = true;
                true
            }
        }
    }

    fn view(&self,_ctx: &Context<Self>,) -> yew::Html {
        if self.sleep {
            sleep()
        } else {
            self.props.children.iter().collect()
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        if _first_render {
            let link = ctx.link().clone();
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
