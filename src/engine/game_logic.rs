use super::Engine;

pub trait GameLogic {
    fn update(&mut self, engine: &mut Engine<Self>)
    where
        Self: Sized;

    fn is_running(&self) -> bool;
}
