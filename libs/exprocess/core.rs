pub trait ExprocessCore {
    type State;
    type Command;
    type Result;
    fn init() -> Self::State;
    fn resolve(prev: &Self::State, command: &Self::Command) -> Self::Result;
    fn reducer(prev: &Self::State, result: &Self::Result) -> Self::State;
}