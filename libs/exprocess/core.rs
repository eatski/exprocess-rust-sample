pub trait ExprocessCore {

    type State;
    type Command;
    type Result;
    ///
    /// Initializes and returns the state of the application.
    /// 
    fn init() -> Self::State;
    ///
    /// Returns a Result from Command and the current state.
    /// This function does not need to be idempotent. 
    /// For example, it is possible to incorporate a random number into the logic.
    /// 
    fn resolve(state: &Self::State, command: Self::Command) -> Self::Result;
    ///
    /// Update the state of the application from Result.
    /// This operation must be idempotent.
    /// By the idempotence, Exprocess can reconstruct the state of the application from the history of Result and this function.
    /// 
    fn reducer(state: &mut Self::State, result: Self::Result);
}


/* sample code */

struct SampleCore;

struct SampleState {
    pub health: i32,
    pub sociability: Sociability
}

enum Sociability {
    #[allow(dead_code)]
    Popular,
    Solitude
}

enum SampleCommand {
    #[allow(dead_code)]
    EatSteak,
    #[allow(dead_code)]
    MeetFriend
}

enum SampleResult {
    FeelBetter(i32),FeelWorse(i32)
}

impl ExprocessCore for SampleCore {
    type State = SampleState;

    type Command = SampleCommand;

    type Result = SampleResult;

    fn init() -> Self::State {
        SampleState {
            health: 100,
            sociability: Sociability::Solitude
        }
    }

    fn resolve(state: &Self::State, command: Self::Command) -> Self::Result {
        match command {
            SampleCommand::EatSteak => {
                SampleResult::FeelBetter(10)
            },
            SampleCommand::MeetFriend => {
                match &state.sociability {
                    Sociability::Popular => SampleResult::FeelBetter(30),
                    Sociability::Solitude => SampleResult::FeelWorse(10),
                }
                
            },
        }
    }

    fn reducer(state: &mut Self::State, result: Self::Result) {
        match result {
            SampleResult::FeelBetter(helth) => {
                state.health = helth;
            },
            SampleResult::FeelWorse(helth) => {
                state.health = -helth;
            },
        }
    }

}
