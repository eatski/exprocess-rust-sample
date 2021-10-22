use exprocess::core::ExprocessCore;
use rand::prelude::SliceRandom;
use serde::{Deserialize, Serialize};

type MemberId = String;

//FIXME: 本来は名前やIDなどの情報は辞書を作成し、数値型で管理するのが理想
#[derive(Serialize, Deserialize,Clone)]
pub struct Member {
    pub name: String,
    pub id: MemberId
}

#[derive(Serialize, Deserialize,Clone)]
pub struct Role {
    pub name: String
}


pub struct AppState {
    pub content: AppStateContent
}

pub enum AppStateContent {
    Blank,
    Standby(Vec<Member>),
    Picked(PickedState)
}

pub struct PickedState {
    pub picked: Vec<(Member,Role)>
}

//FIXME ドメインモデルにつけて問題ない？
#[derive(Serialize, Deserialize)]
pub enum AppCommand {
    Init(Vec<Member>),
    Pick(PickCommand)
}

#[derive(Serialize, Deserialize)]
pub struct PickCommand {
    pub roles: Vec<(usize,Role)>
}

//FIXME ドメインモデルにつけて問題ない？
#[derive(Serialize, Deserialize)]
pub enum AppResult {
    Init(Vec<Member>),
    Picked(PickResult)
}

#[derive(Serialize, Deserialize)]
pub struct PickResult {
    picked: Vec<(usize,Role)>
}

pub struct AppCore;
impl ExprocessCore for AppCore {
    type State = AppState;
    type Command = AppCommand;
    type Result = AppResult;

    fn init() -> Self::State {
        AppState {
            content: AppStateContent::Blank
        }
    }

    fn resolve(prev: &Self::State,command: &Self::Command) -> Self::Result {
        match command {
            AppCommand::Init(members) => AppResult::Init(members.clone()),
            AppCommand::Pick(pick) => {
                match &prev.content {
                    AppStateContent::Standby(members) => AppResult::Picked(pick_roles_to_members(&members,pick)),
                    _ => panic!(),
                }
            },
        }
    }

    fn reducer(prev: &mut Self::State, result: &Self::Result) {
        prev.content = match result {
            AppResult::Init(members) => AppStateContent::Standby(members.clone()),
            AppResult::Picked(result) => {
                match &prev.content {
                    AppStateContent::Standby(members) => {
                        AppStateContent::Picked(PickedState {
                            picked: result.picked.iter().map(move |(index,role)| {
                                let index = *index;
                                let member = members.get(index).expect("Never");
                                (member.clone(),role.clone())
                            }).collect()
                        })
                    },
                    _ => todo!(),
                }
            }
        }
    }
}

fn shuffule<T>(vec:&mut Vec<T>) {
    let mut rng = rand::thread_rng();
    vec.shuffle(&mut rng);
}

fn pick_roles_to_members(members: &Vec<Member>,pick: &PickCommand) -> PickResult {
    let mut roles : Vec<_> = pick.roles
        .iter()
         // FIXME: なんかきもい
        .flat_map(|(num,role)| (vec![role]).repeat(*num))
        .map(|r| r.clone())
        .collect();
    shuffule(&mut roles);
    let picked = (0..(members.len()))
        .map (move |index| {
            let role = roles
                .get(index)
                .expect("Roles Less than Members")
                .clone();
            (
                index,
                role
            )
        })
        .collect();
    PickResult {picked}
}
