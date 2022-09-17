pub use game::MarcAnnoyed;

use log::Level;
use std::panic;
use wasm_bindgen::prelude::*;

mod engine;
mod game;
mod web;

#[wasm_bindgen(start)]
pub fn start() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(Level::Debug);
}
