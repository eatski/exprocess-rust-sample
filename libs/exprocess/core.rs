pub trait ExprocessCore {
    type State;
    type Command;
    type Result;
    fn init() -> Self::State;
    fn resolve(prev: &Self::State, command: Self::Command) -> Self::Result;
    fn reducer(prev: &mut Self::State, result: Self::Result);
}

struct SampleCore;

struct SampleState {
    hoge: Vec<String>,
    fuga: Vec<String>
}

impl ExprocessCore for SampleCore {
    type State = SampleState;

    type Command = ();

    type Result = ();

    fn init() -> Self::State {
        todo!()
    }

    fn resolve(_prev: &Self::State, _command: Self::Command) -> Self::Result {
        todo!()
    }

    fn reducer(prev: &mut Self::State, _result: Self::Result) {
        prev.fuga =  prev.hoge.drain(0..).collect();
    }

}
