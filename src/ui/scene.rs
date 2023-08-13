pub mod start_menu;
pub mod upgrade_character;

use crossterm::event::KeyEvent;
use tui::{backend::Backend, layout::Layout, Frame};

use self::{start_menu::StartMenuScene, upgrade_character::UpgradeCharacterScene};

pub trait Scene {
    fn layout() -> Layout
    where
        Self: Sized,
    {
        Layout::default()
    }
    fn handle_key(&mut self, event: KeyEvent);
    fn render<B: Backend>(&mut self, frame: &mut Frame<B>);
}

#[derive(Debug)]
pub enum SceneType<'a> {
    StartMenu(StartMenuScene<'a>),
    UpgradeCharacter(UpgradeCharacterScene),
}

impl<'a> SceneType<'a> {
    pub fn start_menu() -> Self {
        Self::StartMenu(StartMenuScene::new())
    }

    pub fn handle_key(&mut self, event: KeyEvent) {
        match self {
            Self::StartMenu(scene) => scene.handle_key(event),
            Self::UpgradeCharacter(scene) => scene.handle_key(event),
        }
    }

    pub fn render<B: Backend>(&mut self, frame: &mut Frame<'_, B>) {
        match self {
            Self::StartMenu(scene) => scene.render(frame),
            Self::UpgradeCharacter(scene) => scene.render(frame),
        }
    }
}
