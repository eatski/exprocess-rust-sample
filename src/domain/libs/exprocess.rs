use std::{cell::RefCell, rc::Rc};

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
    fn push(&mut self,record: &Record<Core>);
    fn sync(&mut self,listener: Box<dyn FnMut(Record<Core>)>);
}

pub type Listener<Core: ExprocessCore> = Box<dyn FnMut(&Record<Core>,&Core::State)>;
pub struct Runner<Core: ExprocessCore,Repo: Repository<Core>> {
    repository: Repo,
    shared: Shared<VarsToShare <Core>>
}

struct VarsToShare <Core: ExprocessCore>{
    state: Core::State,
    stack: Vec<String>,
    listener: Listener<Core>
}

type Shared<T> = Rc<RefCell<T>>;

fn shared<T>(content:T) -> Shared<T> { Rc::new(RefCell::new(content))}

//FIXME: ちゃんとやる
impl <Core: ExprocessCore + 'static,Repo: Repository<Core>> Runner<Core,Repo> {
    pub fn start(
        mut repository:Repo,
        listener: Listener<Core>
    ) -> Self {
        let shared = shared(
            VarsToShare {
                state: Core::init(),
                stack: Vec::new(),
                listener
            }
        );
        let cloned = shared.clone();
        repository.sync(Box::new(move |record| {
            let mut shared = cloned.borrow_mut();
            let found = shared.stack
                .iter()
                .enumerate()
                .find(|(_,id)| id.as_str() == record.id);
            match found {
                Some((index,_)) => {shared.stack.remove(index);},
                None => {
                    let next = Core::reducer(&shared.state, record.result);
                    (shared.listener)(&record,&next);
                    shared.state = next;
                }
            };
        }));
        let cloned = shared.clone();
        Self {
            shared:cloned,
            repository
        }
    }
    pub fn dispatch(&mut self,command: Core::Command){
        let mut shared = self.shared.borrow_mut();
        let result = &Core::resolve(&shared.state, &command);
        let id = Uuid::new_v4().to_hyphenated().to_string();
        let id2 = id.clone();
        let record = Record {
            id:id.as_str(),
            result,
            command:&command,
        };
        self.repository.push(&record);
        shared.stack.push(id2);
        let next = Core::reducer(&shared.state,result);
        (shared.listener)(&record,&next);
        shared.state = next;
    }

}