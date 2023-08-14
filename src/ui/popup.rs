use tui::{
    prelude::*,
    widgets::{block::Title, Block, Borders, Clear, Widget},
};

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

pub fn show_popup<B: Backend>(title: &str, widget: impl Widget, frame: &mut Frame<B>) {
    let block = Block::default()
        .title(Title::from(title).alignment(Alignment::Center))
        .borders(Borders::ALL);
    let area = centered_rect(60, 20, frame.size());
    // override this area with clear background
    frame.render_widget(Clear, area);
    frame.render_widget(widget, area);
    frame.render_widget(block, area);
}
