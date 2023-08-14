use std::error;

use crossterm::event::KeyEvent;
use tui::{backend::Backend, Frame};

use crate::{event::EventHandler, setting::Setting, ui::scene::SceneType};

/// Game result type.
pub type GameResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App<'a> {
    /// Is the application running?
    pub running: bool,
    /// Setting
    pub setting: Setting,
    /// Scene
    pub scene: SceneType<'a>,
    /// Event handler
    pub events: EventHandler,
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        Self {
            running: true,
            setting: Setting::default(),
            scene: SceneType::start_menu(),
            events: EventHandler::new(),
        }
    }
}

impl<'a> App<'a> {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Initiate logger and event threads.
    pub fn init(&self, tick_rate: u64) -> GameResult<()> {
        self.events.start(tick_rate)?;
        // TODO add logger initiation here.
        Ok(())
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

    /// Push new scene and replace existing scene.
    pub fn push_scene(&mut self, scene: SceneType<'a>) {
        self.scene = scene;
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        let next_actions = self.scene.handle_key(key);
        self.events.spawn_action_task(next_actions);
    }

    /// Draw current scene.
    pub fn render<B: Backend>(&mut self, frame: &mut Frame<'_, B>) {
        self.scene.render(frame);
    }
}
