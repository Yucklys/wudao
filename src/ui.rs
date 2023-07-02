pub mod component;
pub mod scene;
pub mod selectables;
pub mod text_board;

use tui::{backend::Backend, Frame};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    app.scene.render(frame);
}
