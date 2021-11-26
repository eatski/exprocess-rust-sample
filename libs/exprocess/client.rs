
use std::{cell::RefCell,rc::Rc};

use uuid::Uuid;

use crate::core::ExprocessCore;
pub struct RecordSync<'a,Core: ExprocessCore> {
    pub command: &'a Core::Command,
    pub result: &'a Core::Result,
    pub id: &'a str
}
pub struct Record<Core: ExprocessCore> {
    pub command: Core::Command,
    pub result: Core::Result,
    pub id: String
}
pub trait Repository<Core: ExprocessCore,Err> {
    fn push(&mut self,record: Record<Core>) -> Result<(),Err>;
    fn sync(&mut self,listener: Box<dyn FnMut(Vec<RecordSync<Core>>)>,on_error: Box<dyn FnMut(Err)>);
    fn unsync(&mut self);
}

pub type Listener<Core,State> = Box<dyn FnMut(Vec<RecordSync<Core>>,&State)>;
pub struct Runner<Core: ExprocessCore,Err> {
    repository: Box<dyn Repository<Core,Err>>,
    state: Shared<Core::State>,
    on_error: Shared<Box<dyn FnMut(Err)>>
}

type Shared<T> = Rc<RefCell<T>>;

fn shared<T>(content:T) -> Shared<T> { Rc::new(RefCell::new(content))}

//FIXME: ちゃんとやる
impl <Core: ExprocessCore + 'static,Err : 'static> Runner<Core,Err> where Core::Result : Clone, Core::Command : Clone {

    pub fn start<Repo: Repository<Core,Err> + 'static>(
        mut repository:Repo,
        mut listener: Listener<Core,Core::State>,
        on_error: Box<dyn FnMut(Err)>
    ) -> Self {
        let shared_state = shared(Core::init());
        let cloned = shared_state.clone();
        let on_error = shared(on_error);
        let cloned_on_err = on_error.clone();
        repository.sync(
            Box::new(move |records| {
            let mut shared = cloned.borrow_mut();
            for record in records.iter() {
                Core::reducer(&mut shared ,record.result.clone());
            }
            (listener)(records,&shared);
        }),
            Box::new(move |err| cloned_on_err.borrow_mut()(err)) 
        );
        Self {
            state:shared_state,
            repository: Box::new(repository),
            on_error
        }
    }
    pub fn dispatch(&mut self,command: Core::Command){
        /*
         *  sharedが他の箇所からも借用される変数であり、repository.pushの実装次第では直接別のsharedの参照箇所まで実行される。
         *  そのため、pushが実行される前にsharedを破棄させ、借用制限を守る必要がある
         */ 
        let record = {
            let shared = self.state.borrow();
            let result = Core::resolve(&shared, command.clone());
            let id = Uuid::new_v4().to_hyphenated().to_string();
            Record {
                id,
                result,
                command,
            }
        };
        if let Err(err) = self.repository.push(record) {
            self.on_error.borrow_mut()(err)
        }
    }
    pub fn unsync(&mut self){
        self.repository.unsync();
    }
}