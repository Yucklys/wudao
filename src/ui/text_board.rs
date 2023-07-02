use tui::{
    text::Text,
    widgets::{Block, Borders, Paragraph},
};

use super::component::Component;

#[derive(Debug, Default)]
pub struct TextBoard<'a> {
    content: Text<'a>,
}

impl<'a> Component for TextBoard<'a> {
    fn render<B: tui::backend::Backend>(
        &mut self,
        frame: &mut tui::Frame<'_, B>,
        area: tui::layout::Rect,
    ) {
        let board =
            Paragraph::new(self.content.clone()).block(Block::default().borders(Borders::ALL));
        frame.render_widget(board, area);
    }
}

impl<'a> TextBoard<'a> {
    pub fn new<S: Into<String>>(content: S) -> Self {
        Self {
            content: Text::from(content.into()),
        }
    }
}
