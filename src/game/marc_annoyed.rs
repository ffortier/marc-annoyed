use std::{cell::RefCell, rc::Rc};

use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use web_sys::HtmlElement;

use crate::{
    engine::{Engine, GameLogic},
    web::{AnimationFrame, Canvas, FrameResult},
};

use super::logic::MarcAnnoyedLogic;

#[wasm_bindgen(typescript_custom_section)]
const ITEXT_STYLE: &'static str = r#"
interface GameOptions {
    parent?: HTMLElement
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "GameOptions")]
    pub type GameOptions;

    #[wasm_bindgen(method, getter)]
    pub fn parent(this: &GameOptions) -> Option<HtmlElement>;
}

#[wasm_bindgen]
pub struct MarcAnnoyed {
    engine: Rc<RefCell<Option<Engine<MarcAnnoyedLogic>>>>,
    animation_frame: Rc<RefCell<Option<AnimationFrame>>>,
    canvas: Canvas,
}

#[wasm_bindgen]
impl MarcAnnoyed {
    #[wasm_bindgen(constructor)]
    pub fn new(_options: &GameOptions) -> Self {
        Self {
            engine: Rc::new(RefCell::new(None)),
            animation_frame: Rc::new(RefCell::new(None)),
            canvas: Canvas::default(),
        }
    }

    /// Runs the game
    pub fn run(&mut self) {
        let animation_frame = self.animation_frame.clone();
        let engine = self.engine.clone();
        let game_logic = MarcAnnoyedLogic::default();

        engine.borrow_mut().replace(Engine::new(game_logic));

        animation_frame
            .borrow_mut()
            .replace(AnimationFrame::new(move || {
                engine.borrow_mut().as_mut().unwrap().game_loop()
            }));
    }
}
