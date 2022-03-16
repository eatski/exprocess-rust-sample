use std::{iter::repeat, collections::HashMap, hash::Hash};

use exprocess::core::ExprocessCore;
use rand::{Rng, prelude::SliceRandom};
use serde::{Deserialize, Serialize};

type MemberId = String;

//FIXME: 本来は名前やIDなどの情報は辞書を作成し、数値型で管理するのが理想
#[derive(Serialize, Deserialize,Clone)]
pub struct Member {
    pub id: MemberId,
    pub name: String,
}

#[derive(Serialize, Deserialize,Clone,Hash,PartialEq,Eq)]
pub struct MemberKey(String);

#[derive(Serialize, Deserialize,Clone)]
pub struct Role {
    pub name: String
}

#[derive(Serialize, Deserialize,Clone,Hash,PartialEq,Eq)]
pub struct RoleKey(usize);

pub enum AppState {
    Blank,
    Setting(HashMap<MemberKey,Member>),
    Started(Setting,Started),
}

pub struct Setting {
    pub members: HashMap<MemberKey,Member>,
    pub roles: HashMap<RoleKey,ItemAndHowMany<Role>>
}

pub enum Started {
    Standby,
    Picked(HashMap<MemberKey,RoleKey>)
}

impl Default for AppState {
    fn default() -> Self {
        AppState::Blank
    }
}

#[derive(Serialize,Deserialize,Clone)]
pub enum AppCommand {
    Init(Vec<Member>),
    SetRole(SetRole),
    Pick
}

pub type SetRole = Vec<ItemAndHowMany<Role>>;


type ItemAndHowMany<Item> = (usize,Item);

#[derive(Serialize, Deserialize,Clone)]
pub enum AppResult {
    Init(Vec<Member>),
    SetRole(Vec<ItemAndHowMany<Role>>),
    Picked(PickResult)
}

#[derive(Serialize, Deserialize,Clone)]
pub struct PickResult {
    picked: HashMap<MemberKey,RoleKey>
}

pub struct AppCore;
impl ExprocessCore for AppCore {
    type State = AppState;
    type Command = AppCommand;
    type Result = AppResult;
    ///
    /// It simply returns a blank state.
    /// Originally, we need to pass the data of the members to initialize this application, 
    /// but "currently" Exprocess cannot pass arguments to init.
    /// So, they implemented it to create a blank state once and pass the data as Command later.
    /// 
    fn init() -> Self::State {
        AppState::Blank
    }
    ///
    /// Takes as input roles and the number of members to be assigned to each role, and assigns the roles to the members.
    /// It is important to note that all that is happening in this function is to return the result of assigning the role.
    /// No state update will occur here.
    /// 
    fn resolve(state: &Self::State,command: Self::Command) -> Self::Result {
        match command {
            AppCommand::Init(members) => AppResult::Init(members),
            AppCommand::SetRole(roles) => AppResult::SetRole(roles),
            AppCommand::Pick => {
                match state {
                    AppState::Started(setting,_) => AppResult::Picked(
                        PickResult {
                            picked: pick_roles_to_members(
                                &setting.members.keys().cloned().collect(),
                                &setting.roles.iter().map(|(key,(num,_))|(*num,key.clone())).collect(),
                                &mut rand::thread_rng()
                            )
                        }
                    ),
                    _ => panic!(),
                }
            },
            
        }
    }
    ///
    /// Update the state based on the result of the assignment.
    /// 
    fn reducer(state: &mut Self::State, result: Self::Result) {
        *state = match result {
            AppResult::Init(members) => AppState::Setting(members.into_iter().map(|member| (MemberKey(member.id.clone()),member)).collect()),
            AppResult::SetRole(roles) => {
                match state {
                    AppState::Setting(members) => AppState::Started(
                        Setting {
                            members: drain(members),
                            roles: roles.into_iter().enumerate().map(|(index,(how_many,role))| (RoleKey(index),(how_many,role))).collect()
                        }, Started::Standby
                    ),
                    _ => panic!(),
                }
                    
            },
            AppResult::Picked(result) => {
                match state {
                    AppState::Started(setting,_) => AppState::Started(
                        Setting {
                            members: drain(&mut setting.members),
                            roles: drain(&mut setting.roles)
                        }, Started::Picked(result.picked)
                    ),
                    _ => todo!(),
                }
            }
            
        };
    }
}

fn drain<K: Eq + Hash,T>(target: &mut HashMap<K,T>) -> HashMap<K,T> {
    target.drain().collect()
}

fn pick_roles_to_members<M: Clone + Eq + Hash,R : Clone,Rn: Rng>(
    members: &Vec<M>,
    roles: &Vec<ItemAndHowMany<R>>,
    mut rng: Rn
) -> HashMap<M,R> {
    let mut roles : Vec<_>= roles
        .iter()
        .flat_map(|(num,role)| repeat(role).take(*num))
        .collect();
    roles.shuffle(&mut rng);
    members.iter().zip(roles.into_iter()).map(|(m,r)| (m.clone(),r.clone())).collect()
}

#[test]
fn test_pick_roles_to_members() {
    let result = pick_roles_to_members(
        &vec!["a","b","c"],&vec![(2,"x"),(2,"y")],rand::rngs::mock::StepRng::new(0, 1)
    );
    assert_eq!(result,[("a","x"),("b","y"),("c","y")].iter().cloned().collect());
}