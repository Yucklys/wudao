use std::error;

use tui::{backend::Backend, Frame};

use crate::{setting::Setting, ui::scene::SceneType};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App<'a> {
    /// Is the application running?
    pub running: bool,
    /// Setting
    pub setting: Setting,
    /// Scene
    pub scene: SceneType<'a>,
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        Self {
            running: true,
            setting: Setting::default(),
            scene: SceneType::start_menu(),
        }
    }
}

impl<'a> App<'a> {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Sync version and environment.
    pub fn sync() {
        let missing_data = Setting::check();
        if !missing_data.is_empty() {
            Setting::create(missing_data).unwrap();
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    /// Draw current scene.
    pub fn render<B: Backend>(&mut self, frame: &mut Frame<'_, B>) {
        self.scene.render(frame);
    }
}
