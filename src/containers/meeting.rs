use presentation::loading::loading;
use presentation::meeting::{GuestForm, meeting_guest, meeting_host};
use presentation::members::Member;

use yew::prelude::*;

use js_bridge::{JSFunctionCleaner, register_member, sync_members};

// for Guest
pub struct Meeting {
    props: Props,
    state: State,
    on_destroy: JSFunctionCleaner
}
enum State {
    Loading,
    Fetched(Fetched)
}

struct Fetched {
    members:Vec<Member>,
    form:GuestForm
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub room_id : String,
    pub on_error: Callback<()>
}

pub enum Msg {
    UpdateMember(Vec<Member>),
    Join {name:String}
}

impl Component for Meeting {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props().clone();
        let update = ctx.link().callback(Msg::UpdateMember);
        let on_error = props.on_error.clone();
        let on_destroy = sync_members(
            props.room_id.as_str(), 
            
                move |members| {
                    let members = 
                        members
                        .iter()
                        .map(|member| Member {name: String::from(member.name), you: member.you})
                        .collect::<Vec<Member>>();
                    update.emit(members)
                }
            ,
            move || on_error.clone().emit(())
        );
        Self {
            props,
            state: State::Loading,
            on_destroy
        }
    }
    
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateMember(members) => {
                let form = 
                    if members.iter().any(|m| m.you) { 
                        GuestForm::Joined
                    } else { 
                        GuestForm::Joinable {
                            join: ctx.link().callback(|name| Msg::Join {name})
                        }
                    };
                self.state = State::Fetched( Fetched {members,form} );
                true
            },
            Msg::Join { name } => {
                match &mut self.state {
                    State::Fetched(fetched)  => {
                        fetched.form = GuestForm::Loading;
                    },
                    _ => panic!()
                }
                let on_error = self.props.on_error.clone();
                register_member(
                    self.props.room_id.as_str(),
                    name.as_str(),
                    move || on_error.emit(())
                );
                true
            },
            
        }
    }

    fn destroy(&mut self, ctx: &Context<Self>) {
        self.on_destroy.clean();
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match &self.state {
            State::Loading => loading(),
            State::Fetched (fetched) => {
                meeting_guest(&fetched.form,&fetched.members)
            },
        }
    }
}

// for Host

pub struct MeetingHost {
    props: PropsHost,
    state: StateHost,
    on_destroy: JSFunctionCleaner
}

#[derive(Clone, Properties, PartialEq)]
pub struct PropsHost {
    pub room_id : String,
    pub start: Callback<()>,
    pub on_error: Callback<()>,
}

enum StateHost {
    Loading,
    Fetched {
        members:Vec<Member>,
    }
}

pub enum MsgHost {
    UpdateMember(Vec<Member>)
}

impl Component for MeetingHost {

    type Message = MsgHost;
    type Properties = PropsHost;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props().clone();
        let update = ctx.link().callback(MsgHost::UpdateMember);
        let on_error = props.on_error.clone();
        let on_destroy = sync_members(
            props.room_id.as_str(), 
                move |members| {
                    let members = 
                        members
                        .iter()
                        .map(|member| Member {name: String::from(member.name), you: member.you})
                        .collect();
                    update.emit(members)
                }
            ,
            move || on_error.clone().emit(())
        );
        Self {
            props,
            state: StateHost::Loading,
            on_destroy
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MsgHost::UpdateMember(members) => {
                self.state = StateHost::Fetched {members};
                true
            }
        }
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {
        self.on_destroy.clean();
    }

    fn view(&self, _ctx: &Context<Self>, ) -> Html {
        match &self.state {
            StateHost::Loading => loading(),
            StateHost::Fetched { members } => {
                meeting_host(members,&self.props.start.reform(|_| ()))
            },
        }
        
    }
}
