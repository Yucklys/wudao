use tui::{backend::Backend, layout::Rect, Frame};

pub trait Component {
    fn render<B: Backend>(&mut self, frame: &mut Frame<'_, B>, area: Rect);
}
