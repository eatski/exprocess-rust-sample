use uuid::Uuid;

pub trait ExprocessCore {
    type State;
    type Command;
    type Result;
    fn init() -> Self::State;
    fn resolve(prev: &Self::State, command: &Self::Command) -> Self::Result;
    fn reducer(prev: &Self::State, result: &Self::Result) -> Self::State;
}

pub struct Record<'a,Core: ExprocessCore> {
    pub command: &'a Core::Command,
    pub result: &'a Core::Result,
    pub id: &'a str
}
pub trait Repository<Core: ExprocessCore> {
    fn start(listener: Box<dyn FnMut(Record<Core>)>) -> Self;
    fn push(&mut self,record: &Record<Core>);
}

pub type Listener<Core: ExprocessCore> = Box<dyn FnMut(Record<Core>,Core::State)>;
pub struct Runner<Core: ExprocessCore,Repo: Repository<Core>> {
    state: StateWrapper<Core::State>,
    repository: Repo,
    stack: Vec<String>,
    listener: Listener<Core>
}
struct StateWrapper<State> {
    value: State
}
//FIXME: ちゃんとやる
impl <Core: ExprocessCore + 'static,Repo: Repository<Core>> Runner<Core,Repo> {
    pub fn start(
        listener: Listener<Core>
    ) -> Self {
        let mut state = StateWrapper { value:Core::init() };
        let mut stack :Vec<String>= Vec::new();
        let repository = Repo::start( Box::new(move |record| {
            let found = stack
                .iter()
                .enumerate()
                .find(|(_,id)| id.as_str() == record.id);
            match found {
                Some((index,_)) => {stack.remove(index);},
                None => {
                    let next = Core::reducer(&state.value, record.result);
                    state.value = next;
                }
            };
        }));
        todo!();
        Self {
            state,
            repository,
            stack,
            listener
        }
    }
    pub fn dispatch(&mut self,command: Core::Command){
        let state = &self.state.value;
        let result = &Core::resolve(state, &command);
        let id = Uuid::new_v4().to_hyphenated().to_string();
        let id2 = id.clone();
        let record = Record {
            id:id.as_str(),
            result,
            command:&command,
        };
        self.repository.push(&record);
        self.stack.push(id2);
        let state = Core::reducer(state,result);
        (self.listener)(record,state);
    }

}