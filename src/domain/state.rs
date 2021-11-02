use exprocess::core::ExprocessCore;
use rand::{Rng, prelude::SliceRandom};
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
#[derive(Serialize, Deserialize,Clone)]
pub enum AppCommand {
    Init(Vec<Member>),
    Pick(PickCommand)
}


type ItemAndHowMany<Item> = (usize,Item);
#[derive(Serialize, Deserialize,Clone)]
pub struct PickCommand {
    pub roles: Vec<ItemAndHowMany<Role>>
}

//FIXME ドメインモデルにつけて問題ない？
#[derive(Serialize, Deserialize,Clone)]
pub enum AppResult {
    Init(Vec<Member>),
    Picked(PickResult)
}

#[derive(Serialize, Deserialize,Clone)]
pub struct PickResult {
    picked: Vec<Role>
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

    fn resolve(prev: &Self::State,command: Self::Command) -> Self::Result {
        match command {
            AppCommand::Init(members) => AppResult::Init(members.clone()),
            AppCommand::Pick(command) => {
                match &prev.content {
                    AppStateContent::Standby(members) => AppResult::Picked(
                        PickResult {
                            picked: pick_roles_to_members(
                                &members,
                                command.roles,
                                &mut rand::thread_rng()
                            )
                        }
                    ),
                    _ => panic!(),
                }
            },
        }
    }

    fn reducer(prev: &mut Self::State, result: Self::Result) {
        prev.content = match result {
            AppResult::Init(members) => AppStateContent::Standby(members.clone()),
            AppResult::Picked(result) => {
                match &mut prev.content {
                    AppStateContent::Standby(members) => {
                        AppStateContent::Picked(PickedState {
                            picked: result
                                .picked
                                .into_iter()
                                .map(move |role| (members.remove(0),role))
                                .collect()
                        })
                    },
                    _ => todo!(),
                }
            }
        };
    }
}

fn pick_roles_to_members<M,R : Clone,Rn: Rng>(
    members: &Vec<M>,
    roles: Vec<ItemAndHowMany<R>>,
    mut rng: Rn
) -> Vec<R> {
    let mut roles : Vec<_>= roles
        .iter()
         // FIXME: なんかきもい
        .flat_map(|(num,role)| (vec![role]).repeat(*num))
        .collect();
    roles.shuffle(&mut rng);
    (0..(members.len()))
        .map (move |_| roles.remove(0).clone())
        .collect()
}

#[test]
fn test_pick_roles_to_members() {
    let result = pick_roles_to_members(
        &vec!["a","b","c"],vec![(2,"x"),(1,"y")],rand::rngs::mock::StepRng::new(0, 1)
    );
    assert_eq!(result,["x","y","x"])
}
