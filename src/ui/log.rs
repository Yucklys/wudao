use tui::{
    prelude::*,
    widgets::{Block, Borders},
};
use tui_logger::{TuiLoggerLevelOutput, TuiLoggerWidget};

#[derive(Debug, Default)]
pub struct LogPopup;

impl LogPopup {
    pub fn widget<'a>() -> TuiLoggerWidget<'a> {
        TuiLoggerWidget::default()
            .block(Block::default().borders(Borders::ALL))
            .output_separator('|')
            .output_timestamp(Some("%F %H:%M:%S".to_string()))
            .output_level(Some(TuiLoggerLevelOutput::Long))
            .output_target(false)
            .output_file(false)
            .output_line(false)
            .style(Style::default().fg(Color::White))
            .style_info(Style::default().fg(Color::Green))
    }
}
