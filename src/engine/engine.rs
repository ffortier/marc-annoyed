use std::{cell::RefCell, collections::HashMap, hash::Hash, rc::Rc};

use web_sys::FontFace;

use crate::web::FrameResult;

use super::GameLogic;

/// Controls the game loop, the game objects and the events
pub struct Engine<T>
where
    T: GameLogic,
{
    game_logic: Rc<RefCell<T>>,
    fonts: HashMap<String, LoadState<FontFace>>,
}

enum LoadState<T> {
    Loading,
    Loaded(T),
}

impl<T> Engine<T>
where
    T: GameLogic,
{
    pub fn new(game_logic: T) -> Self {
        Self {
            game_logic: Rc::new(RefCell::new(game_logic)),
            fonts: HashMap::new(),
        }
    }

    pub fn game_loop(&mut self) -> FrameResult {
        let game_logic = self.game_logic.clone();

        game_logic.borrow_mut().update(self);

        match self.game_logic.borrow().is_running() {
            true => FrameResult::Continue,
            _ => FrameResult::Stop,
        }
    }

    pub fn load_font(&mut self, name: &str) {
        self.fonts.get(name);
    }
}
