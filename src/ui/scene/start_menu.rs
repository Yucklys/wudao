use crate::{
    setting::read_file,
    ui::{
        component::Component,
        selectables::{Selectable, SelectableList},
        text_board::TextBoard,
    },
};
use crossterm::event::{KeyCode, KeyEvent};
use tui::prelude::*;

use super::Scene;

#[derive(Debug)]
pub struct StartMenuScene<'a> {
    start_menu: SelectableList<String>,
    changelog: TextBoard<'a>,
}

impl<'a> StartMenuScene<'a> {
    pub fn new() -> Self {
        let changelog_content = read_file("changelog.txt").unwrap();
        StartMenuScene {
            start_menu: SelectableList::with_items(vec![
                "更新日志".to_string(),
                "开始游戏".to_string(),
                "游戏设置".to_string(),
                "退出游戏".to_string(),
            ]),
            changelog: TextBoard::new(changelog_content),
        }
    }
}

impl<'a> Scene for StartMenuScene<'a> {
    fn layout() -> Layout {
        Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Ratio(1, 4), Constraint::Ratio(3, 4)].as_ref())
    }

    fn render<B: Backend>(&mut self, frame: &mut Frame<'_, B>) {
        let StartMenuScene {
            start_menu,
            changelog,
        } = self;
        let layout = Self::layout();
        let chunks = layout.split(frame.size());

        start_menu.render(frame, chunks[0]);
        if let Some(i) = start_menu.selected() {
            match i {
                0 => changelog.render(frame, chunks[1]),
                _ => {}
            }
        }
    }

    fn handle_key(&mut self, event: KeyEvent) {
        let StartMenuScene { start_menu, .. } = self;
        match event.code {
            KeyCode::Tab | KeyCode::Char('j') => start_menu.next(),
            KeyCode::BackTab | KeyCode::Char('k') => start_menu.prev(),
            _ => {}
        };
    }
}
