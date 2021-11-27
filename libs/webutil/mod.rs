pub mod window {
    use js_sys::{Array, Function};
    use mytil::{Cleanable, Cleaner};
    use wasm_bindgen::prelude::*;
    use web_sys::{Window, window};
    pub fn set_timeout<CB: FnOnce() + 'static>(callback: CB,time_ms: i32) -> Cleaner<ClearTimeout> {
        let callback = Closure::once_into_js( callback);
        let window = window().unwrap();
        let id = window
            .set_timeout_with_callback_and_timeout_and_arguments(&callback.into(), time_ms, &Array::new())
            .expect("JS Error");
        ClearTimeout {
            window,
            id
        }.into()
    }

    pub struct ClearTimeout {
        window: Window,
        id: i32
    }

    impl Cleanable for ClearTimeout {
        fn clean(self) {
            self.window.clear_timeout_with_handle(self.id)
        }
    }
    
    pub fn add_eventlistener<CB: FnMut() + 'static>(name: &str,callback: CB) -> Cleaner<RemoveListener> {
        let callback : Box<dyn FnMut()> = Box::new(callback);
        let callback = Closure::wrap( callback).into_js_value().into();
        let window = window().unwrap();
        window
            .add_event_listener_with_callback(name, &callback)
            .expect("JS Error");
        let name = name.to_string();
        RemoveListener {
            window,
            func: callback,
            name
        }.into()
    }
    pub struct RemoveListener {
        window: Window,
        func: Function,
        name: String
    }

    impl Cleanable for RemoveListener {
        fn clean(self) {
            self.window
            .remove_event_listener_with_callback(self.name.as_str(), &self.func)
            .expect("JS Error")
        }
    }

}


pub mod util {
    use std::{cell::{RefCell}, rc::Rc};

    use mytil::Cleaner;

    use crate::window::{ClearTimeout, add_eventlistener};

    use super::window::set_timeout;
    /**
     * callbackを発火し、一定期間そのcallbackの実行を止める
     */
    pub fn stop_interval(mut callback:Box<dyn FnMut()>,interval: i32) -> Box<dyn FnMut()> {
        let stopping = Rc::new(RefCell::new(false));
        Box::new(move || {
            if !*stopping.borrow() {
                callback();
                stopping.replace(true);
                let stopping = stopping.clone();
                set_timeout(move || { stopping.replace(false); }, interval).ignore();
            }
        })
    }

    pub struct ResetableTimer<CB: FnMut()> {
        callback: Rc<RefCell<CB>>,
        ms: i32,
        clear_inner: Option<Cleaner<ClearTimeout>>
    }
    
    impl <CB: FnMut() + 'static>ResetableTimer<CB> {
        pub fn start(&mut self) {
            let callback = self.callback.clone();
            let clear = self.clear_inner.replace(
                set_timeout(move || callback.borrow_mut()(), self.ms)
            );
            clear.map(|mut cleaner| cleaner.clean());
        }
        pub fn clear(&mut self) {
            self.clear_inner.take().map(|mut cleaner| cleaner.clean());
        }
        pub fn new(callback: CB,ms: i32) -> Self {
            Self {
                callback: Rc::new(RefCell::new(callback)),
                ms,
                clear_inner: None
            }
        }
    }

    pub fn set_timeout_no_mousemove<F: FnMut() + 'static>(callback: F,ms: i32,mouse_move_interval: i32) -> Box<dyn FnOnce()> {
        let timer = Rc::new(RefCell::new(ResetableTimer::new(callback,ms)));
        let cloned_timer = timer.clone();
        let on_mousemove = stop_interval(Box::new(move || {
            cloned_timer.borrow_mut().start();
        }), mouse_move_interval);
        let mut remove_eventlistener = add_eventlistener("mousemove", on_mousemove);
        Box::new(move || {
            remove_eventlistener.clean();
            timer.borrow_mut().clear();
        })
    }
}

