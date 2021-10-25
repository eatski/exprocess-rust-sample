use core::panic;
use std::{cell::RefCell, rc::Rc};

pub trait ExprocessState<Root> {
    type Command;
    type Result;
    fn resolve(prev: &Self, command: &Self::Command) -> Self::Result;
    fn reducer(prev: &Self, result: &Self::Result) -> Root;
}

pub enum ReducedState<Slf,Root> {
    Moved(Root),
    Stay(Slf)
}

pub trait ExprocessStateAggregationRoot {
    fn init() -> Self;
}

pub struct StateStore<Root> {
    listener: RefCell<Rc<Box<dyn FnMut(&Root)>>>
}

impl <Root: 'static>StateStore<Root> {
    pub fn get_interface<State : ExprocessState<Root>>(&self,state: State) -> StateInterface<Root,State>{
        let cloned = self.listener.clone();
        StateInterface {
            state: Some(state),
            listener: Box::new(move |root| {
                let mut listener = cloned.borrow_mut();
                (listener)(root)
            })
        }
    }
}

pub struct StateInterface<Root,State: ExprocessState<Root>> {
    pub state: Option<State>,
    listener: Box<dyn FnMut(&Root)>
}

impl <Root,State: ExprocessState<Root>>StateInterface<Root,State> {
    pub fn dispatch (&mut self,command: State::Command){
        match &self.state {
            Some(state) => {
                let result = State::resolve(state, &command);
                let next = State::reducer(state, &result);
                (self.listener)(&next);
                self.state = None
            },
            None => panic!(),
        }
    }
}

pub enum SampleAggregationRoot {
    Sample0,
    Sample1(Sample1),
    Sample2
}

impl SampleAggregationRoot {
    fn a(&self) {
        match self {
            SampleAggregationRoot::Sample0 => todo!(),
            SampleAggregationRoot::Sample1(_) => todo!(),
            SampleAggregationRoot::Sample2 => todo!(),
        }
    }
}

impl ExprocessStateAggregationRoot for SampleAggregationRoot {
    fn init() -> Self {
        Self::Sample0
    }
}

pub struct Sample1;
