use std::{cell::{RefCell, RefMut}, collections::HashSet, rc::Rc};

use super::exprocess::{ExprocessCore, Record, RecordSync, Repository};
pub struct DirectlyDispatch<Core: ExprocessCore,Inner: Repository<Core>> {
    inner: Inner,
    listener: Shared<Box<dyn FnMut(Vec<RecordSync<Core>>)>>,
    used_id: SharedUsedId
}

type Shared<T> = Rc<RefCell<T>>;

type SharedUsedId = Shared<HashSet<String>>;

impl <Core: ExprocessCore + 'static,Inner: Repository<Core>> Repository<Core> for DirectlyDispatch<Core,Inner> {

    fn push(&mut self,record: Record<Core>) {
        let records = 
            vec![RecordSync {id: record.id.as_str(),command: &record.command, result: &record.result}];
        (self.listener.borrow_mut())(records);
        self.used_id.borrow_mut().insert(record.id.clone());
        self.inner.push(record);
    }

    fn sync(&mut self,listener: Box<dyn FnMut(Vec<RecordSync<Core>>)>) {
        let used_id = self.used_id.clone();
        let shared_listener = shared(listener);
        let listener = shared_listener.clone();
        self.inner.sync(Box::new(move |records| {
            let mut listener = listener.borrow_mut();
            let used_id = used_id.borrow_mut();
            let records = limit_records(records, used_id);
            listener(records);
        }));
        self.listener = shared_listener;
    }
}

impl <Core: ExprocessCore,Inner: Repository<Core>> DirectlyDispatch<Core,Inner> {
    pub fn wrap(inner: Inner) -> Self {
        Self {
            inner,
            listener: shared(Box::new(|_|panic!("No Listener"))),
            used_id: shared(HashSet::new())
        }
    }
}

fn shared<T>(content:T) -> Rc<RefCell<T>> { Rc::new(RefCell::new(content))}

fn limit_records<'a,Core: ExprocessCore>(
    records: Vec<RecordSync<'a,Core>>,
    mut used_id: RefMut<HashSet<String>>
) -> Vec<RecordSync<'a,Core>> {
    records.into_iter().filter(move |record| !used_id.remove(record.id)).collect()
}

struct TestCore;

impl ExprocessCore for TestCore {
    type State = ();

    type Command = ();

    type Result = ();

    fn init() -> Self::State {
        todo!()
    }

    fn resolve(_prev: &Self::State, _command: &Self::Command) -> Self::Result {
        todo!()
    }

    fn reducer(_prev: &Self::State, _result: &Self::Result) -> Self::State {
        todo!()
    }
}

fn _test_limit_records() {
    let hashset: HashSet<String> = HashSet::new();
    let shared_hashset = shared(hashset);
    {
        let mut borrowed = shared_hashset.borrow_mut();
        borrowed.insert(String::from("used"));
    }
    {
        let borrowed = shared_hashset.borrow_mut();
        let sample_record: Vec<RecordSync<TestCore>> = vec![
            RecordSync {id: "used",result:&(),command:&()},
            RecordSync {id: "unused",result:&(),command:&()},
        ];
        let result = limit_records(sample_record, borrowed);
        assert_eq!(result.len(),1);
        assert_eq!(result.first().unwrap().id,"unused");
    }
    {
        let borrowed = shared_hashset.borrow();
        assert_eq!(borrowed.len(),0);
    }
}

#[test]
fn test() {
    _test_limit_records() 
}