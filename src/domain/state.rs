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

pub enum AppState {
    Blank,
    Standby(Vec<Member>),
    Picked(PickedState)
}

impl Default for AppState {
    fn default() -> Self {
        AppState::Blank
    }
}

pub struct PickedState {
    pub picked: Vec<(Member,Role)>
}

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
            AppCommand::Pick(command) => {
                match state {
                    AppState::Standby(members) => AppResult::Picked(
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
    ///
    /// Update the state based on the result of the assignment.
    /// 
    fn reducer(state: &mut Self::State, result: Self::Result) {
        *state = match result {
            AppResult::Init(members) => AppState::Standby(members),
            AppResult::Picked(result) => {
                match state {
                    AppState::Standby(members) => {
                        AppState::Picked(PickedState {
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