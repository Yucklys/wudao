use crossterm::event::{KeyCode, KeyEvent};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::setting::read_file;

use super::{
    component::Component,
    selectables::{Selectable, SelectableList},
    text_board::TextBoard,
};

#[derive(Debug)]
pub enum Scene<'a> {
    StartMenuScene {
        start_menu: SelectableList<String>,
        changelog: TextBoard<'a>,
        layout: Layout,
    },
}

impl<'a> Scene<'a> {
    pub fn start_menu() -> Self {
        let changelog_content = read_file("changelog.txt").unwrap();
        Self::StartMenuScene {
            start_menu: SelectableList::with_items(vec![
                "更新日志".to_string(),
                "开始游戏".to_string(),
                "游戏设置".to_string(),
                "退出游戏".to_string(),
            ]),
            changelog: TextBoard::new(changelog_content),
            layout: Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([Constraint::Ratio(1, 4), Constraint::Ratio(3, 4)].as_ref()),
        }
    }

    pub fn render<B: Backend>(&mut self, frame: &mut Frame<'_, B>) {
        match self {
            Self::StartMenuScene {
                start_menu,
                changelog,
                layout,
            } => {
                let chunks = layout.split(frame.size());

                start_menu.render(frame, chunks[0]);
                if let Some(i) = start_menu.selected() {
                    match i {
                        0 => changelog.render(frame, chunks[1]),
                        _ => {}
                    }
                }
            }
        }
    }

    pub fn keybinding(&mut self, key_event: KeyEvent) {
        match self {
            Self::StartMenuScene {
                start_menu,
                changelog: _,
                layout: _,
            } => match key_event.code {
                KeyCode::Tab | KeyCode::Char('j') => start_menu.next(),
                KeyCode::BackTab | KeyCode::Char('k') => start_menu.prev(),
                _ => {}
            },
        }
    }
}
