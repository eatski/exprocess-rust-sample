pub mod window {
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = window,js_name = setTimeout)]
        fn set_timeout_js(callback:JsValue,time_ms: u32) -> u32;

        #[wasm_bindgen(js_namespace = window,js_name = clearTimeout)]
        fn clear_timeout_js(id:u32);

        #[wasm_bindgen(js_namespace = window,js_name = addEventListener)]
        fn add_eventlistener_js(name: &str,callback:&JsValue);

        #[wasm_bindgen(js_namespace = window,js_name = removeEventListener)]
        fn remove_eventlistener_js(name: &str,callback:&JsValue);
    }

    pub fn set_timeout<CB: FnMut() + 'static>(callback: CB,time_ms: u32) -> Box<dyn FnMut()>{
        let callback : Box<dyn FnMut()> = Box::new(callback);
        let callback = Closure::wrap( callback).into_js_value();
        let id = set_timeout_js(callback,time_ms);
        Box::new(move || clear_timeout_js(id))
    }
    
    pub fn add_eventlistener<CB: FnMut() + 'static>(name: &str,callback: CB) -> Box<dyn FnMut()>{
        let callback : Box<dyn FnMut()> = Box::new(callback);
        let callback = Closure::wrap( callback).into_js_value();
        add_eventlistener_js(name,&callback);
        let name = name.to_owned();
        Box::new(move || remove_eventlistener_js(name.as_str(),&callback))
    }
}

pub mod util {
    use std::{cell::RefCell, rc::Rc};

    use mytil::{SingletonStateContainer};
    use crate::window::add_eventlistener;

    use super::window::set_timeout;
    /**
     * callbackを発火し、一定期間そのcallbackの実行を止める
     */
    pub fn stop_interval(mut callback:Box<dyn FnMut()>,interval: u32) -> Box<dyn FnMut()> {
        let stopping = SingletonStateContainer::new(false);
        Box::new(move || {
            if !stopping.get() {
                callback();
                stopping.set(true);
                let stopping = stopping.clone();
                let _ = set_timeout(move || stopping.set(false), interval);
            }
        })
    }

    pub struct ResetableTimer<CB: FnMut()> {
        callback: Rc<RefCell<CB>>,
        ms: u32,
        clear_inner: Box<dyn FnMut()>
    }
    
    impl <CB: FnMut() + 'static>ResetableTimer<CB> {
        pub fn start(&mut self) {
            (self.clear_inner)();
            let callback = self.callback.clone();         
            self.clear_inner = set_timeout(move || callback.borrow_mut()(), self.ms);
        }
        pub fn clear(&mut self) {
            (self.clear_inner)();
            self.clear_inner = Self::noop();
        }

        fn noop() -> Box<dyn FnMut()>{
            Box::new(|| ())
        }
        pub fn new(callback: CB,ms: u32) -> Self {
            Self {
                callback: Rc::new(RefCell::new(callback)),
                ms,
                clear_inner: Self::noop()
            }
        }
    }

    pub fn set_timeout_no_mousemove<F: FnMut() + 'static>(callback: F,ms: u32,mouse_move_interval: u32) -> Box<dyn FnMut()> {
        let timer = Rc::new(RefCell::new(ResetableTimer::new(callback,ms)));
        let cloned_timer = timer.clone();
        let on_mousemove = stop_interval(Box::new(move || {
            cloned_timer.borrow_mut().start();
        }), mouse_move_interval);
        let mut remove_eventlistener = add_eventlistener("mousemove", on_mousemove);
        Box::new(move || {
            remove_eventlistener();
            timer.borrow_mut().clear();
        })
    }
}

