use std::{cell::RefCell, rc::Rc};

use js_sys::Function;
use wasm_bindgen::{prelude::Closure, UnwrapThrowExt};
use web_sys::window;

pub struct AnimationFrame {
    callback: Rc<RefCell<Option<Function>>>,
    rid: Rc<RefCell<Option<i32>>>,
}

pub enum FrameResult {
    Continue,
    Stop,
}

impl AnimationFrame {
    pub fn new<G>(frame: G) -> Self
    where
        G: Fn() -> FrameResult + 'static,
    {
        let callback = Rc::new(RefCell::new(None));
        let rid = Rc::new(RefCell::new(None));

        callback.borrow_mut().replace({
            let callback = callback.clone();
            let rid = rid.clone();

            let closure: Closure<dyn FnMut()> = Closure::new(move || {
                match frame() {
                    FrameResult::Continue => {
                        let id = request_animation_frame(callback.borrow().as_ref().unwrap());

                        rid.borrow_mut().replace(id);
                    }
                    FrameResult::Stop => {
                        rid.borrow_mut().take();
                    }
                };
            });

            closure.into_js_value().into()
        });

        let id = request_animation_frame(callback.borrow().as_ref().unwrap());

        rid.borrow_mut().replace(id);

        Self { callback, rid }
    }
}

fn request_animation_frame(callback: &Function) -> i32 {
    window()
        .unwrap_throw()
        .request_animation_frame(callback)
        .unwrap_throw()
}

fn cancel_animation_frame(rid: i32) {
    window()
        .unwrap_throw()
        .cancel_animation_frame(rid)
        .unwrap_throw()
}

impl Drop for AnimationFrame {
    fn drop(&mut self) {
        if let Some(rid) = self.rid.borrow_mut().take() {
            cancel_animation_frame(rid);
        }

        if let Some(callback) = self.callback.borrow_mut().take() {
            drop(callback);
        }
    }
}
