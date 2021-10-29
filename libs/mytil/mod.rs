use std::{cell::{Ref, RefCell, RefMut}, fmt::Display, rc::{Rc, Weak}};

use crate::testing::Counter;

#[derive(Debug)]
pub struct Container<T> {
    pub content: T
}

impl <T>Container<T> {
    pub fn set(&mut self,content: T) {
        self.content = content;
    }
    pub fn get(&self) -> &T {
        &self.content
    }
    pub fn new(content: T) -> Self {
        Self {content}
    }
}

#[derive(Clone)]
pub struct SingletonStateContainer<T> {
    singleton: Rc<RefCell<T>>
}

impl <T>SingletonStateContainer<T> {
    pub fn get_map<R,F: FnOnce(Ref<T>) -> R>(&self,map_fn: F) -> R {
        map_fn(self.singleton.borrow())
    }
    pub fn set(&self,content: T) {
        self.singleton.replace(content);
    }
    pub fn new(content: T) -> Self {
        Self {
            singleton: Rc::new(RefCell::new(content))
        }
    }
    pub fn get(&self) -> T where T : Copy {
        *self.singleton.borrow()
    }
}

pub mod testing {
    use super::*;

    #[derive(Debug,Clone)]
    pub struct Counter {
        num: Rc<RefCell<Container<usize>>>,
    }

    impl Counter {
        pub fn count(&self) {
            let cur = {
                *self.num.borrow().get()
            };
            let mut num = self.num.borrow_mut();
            num.set(cur + 1)
        }
        pub fn get(&self) -> usize {
            *self.num.borrow().get()
        }
        pub fn new() -> Self {
            Self {
                num: Rc::new(RefCell::new(Container::new(0))),
            }
        }
        pub fn map<T, F: FnOnce(Self) -> T>(&self, func: F) -> T {
            func(self.clone())
        }
    }

    impl PartialEq<usize> for Counter {
        fn eq(&self, other: &usize) -> bool {
            self.num.borrow().get() == other
        }
    }

    impl Display for Counter {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", *self.num.borrow().get())
        }
    }

    fn _test_counter() {
        let cnt = Counter::new();
        (0..3)
            .into_iter()
            .map(|_| {
                Box::new(|| {
                    cnt.count();
                }) as Box<dyn FnMut()>
            })
            .for_each(|mut call| call());
        assert_eq!(cnt, 3);
    }

    #[test]
    fn test_counter() {
        _test_counter()
    }
}


struct FnContainer{
    value: Box<dyn FnMut()>
}

impl FnContainer {
    pub fn set(&mut self,value:Box<dyn FnMut()>) {
        self.value = value;
    }
    pub fn call(&mut self) {
        (self.value)()
    }
    pub fn empty() -> Self {
        Self {
            value: Box::new(|| ())
        }
    }
}

type ToOnceListen = Box<dyn FnMut(Box<dyn FnMut()>)-> Box<dyn FnMut()>>;
fn to_once<L: FnMut(Box<dyn FnMut()>)-> Box<dyn FnMut()> + 'static>(mut listen: L) -> ToOnceListen {
    Box::new(move |mut callback| {
        let shared_unlisten = Rc::new(RefCell::new(FnContainer::empty()));
        let cloned_unlisten = shared_unlisten.clone();
        let unlisten = listen(Box::new(move || {
            callback();
            cloned_unlisten.borrow_mut().call();
        }));
        let mut rm = shared_unlisten.borrow_mut();
        rm.set(unlisten);
        let cloned_unlisten = shared_unlisten.clone();
        Box::new(move || cloned_unlisten.borrow_mut().call())
    })
}

fn _test_to_once_call_remove_fn() {
    let caller = Rc::new(RefCell::new(FnContainer::empty()));
    let cloned = caller.clone();
    let cnt_rm = Counter::new();
    let mut listen_once = cnt_rm.map(|cnt_rm| to_once(move |callback|{
        cloned.borrow_mut().set(callback);
        let cloned_cnt_rm = cnt_rm.clone();
        Box::new(move || cloned_cnt_rm.count())
    }));
    let cnt_called = Counter::new();
    let _ = listen_once(
        cnt_called.map(|cnt| Box::new(move || cnt.count()))
    );
    caller.borrow_mut().call();
    assert!(cnt_called == 1);
    assert!(cnt_rm == 1);
}
#[test]
fn test() {
    _test_to_once_call_remove_fn() 
}

#[test]
fn test_call_while_living() {
    _test_call_while_living()
}

fn _test_call_while_living() {
    let counter = Counter::new();
    let mut call = {
        let cnt = counter.map(|counter| Box::new(move || counter.count())  as Box<dyn FnMut()>);
        let rc = Rc::new(RefCell::new(cnt));
        let mut call = call_while_living(&rc, |cnt| cnt.borrow_mut()());
        call();
        assert!(counter == 1);
        call
    };
    call();
    assert!(counter == 1);
}

pub fn call_while_living<T : 'static,CB: FnMut(&T) + 'static>(target: &Rc<T>,mut call: CB) -> Box<dyn FnMut()> {
    let weak = Rc::downgrade(&target);
    Box::new(move || {
        match weak.upgrade() {
            Some(target) => call(&target),
            None => (),
        }
    })
}

