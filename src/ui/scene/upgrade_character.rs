use tui::{
    prelude::*,
    widgets::{Block, Borders},
};

use crate::action::Action;

use super::Scene;

#[derive(Debug, Default)]
pub struct UpgradeCharacterScene {}

impl Scene for UpgradeCharacterScene {
    fn handle_key(&mut self, event: crossterm::event::KeyEvent) -> Vec<Action> {
        let actions = Vec::new();
        match event {
            _ => {}
        }
        actions
    }

    fn render<B: Backend>(&mut self, frame: &mut tui::Frame<B>)
    where
        Self: Sized,
    {
        frame.render_widget(Block::default().borders(Borders::ALL), frame.size());
    }
}
