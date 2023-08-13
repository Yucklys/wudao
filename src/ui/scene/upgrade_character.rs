use tui::prelude::*;

use super::Scene;

#[derive(Debug, Default)]
pub struct UpgradeCharacterScene {}

impl Scene for UpgradeCharacterScene {
    fn handle_key(&mut self, event: crossterm::event::KeyEvent) {
        todo!()
    }

    fn render<B: Backend>(&mut self, frame: &mut tui::Frame<B>)
    where
        Self: Sized,
    {
        todo!()
    }
}
