use std::{cell::RefCell, future::Future, process::Output, rc::Rc};

use crate::engine::{Engine, GameLogic};

pub struct MarcAnnoyedLogic {
    scene: Rc<RefCell<Box<dyn Scene>>>,
}

impl MarcAnnoyedLogic {
    fn change_scene(&mut self, scene: Box<dyn Scene>) {
        self.scene = Rc::new(RefCell::new(scene));
    }
}

impl Default for MarcAnnoyedLogic {
    fn default() -> Self {
        Self {
            scene: Rc::new(RefCell::new(Box::new(LoadingScene::default()))),
        }
    }
}

impl GameLogic for MarcAnnoyedLogic {
    fn update(&mut self, engine: &mut Engine<Self>) {
        log::debug!("update");

        let scene = self.scene.clone();

        scene.borrow_mut().update(self, engine);
    }

    fn is_running(&self) -> bool {
        true
    }
}

trait Scene {
    fn update(&mut self, logic: &mut MarcAnnoyedLogic, engine: &mut Engine<MarcAnnoyedLogic>);
}

#[derive(Debug, Default)]
struct LoadingScene {
    progress: u32,
}

impl Scene for LoadingScene {
    fn update(&mut self, logic: &mut MarcAnnoyedLogic, engine: &mut Engine<MarcAnnoyedLogic>) {
        todo!()
    }
}

#[derive(Debug, Default)]
struct SplashScene {}

impl Scene for SplashScene {
    fn update(&mut self, logic: &mut MarcAnnoyedLogic, engine: &mut Engine<MarcAnnoyedLogic>) {}
}
