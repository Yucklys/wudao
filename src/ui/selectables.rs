use std::fmt::Display;

use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

use super::component::Component;

pub trait Selectable<T> {
    fn next(&mut self);
    fn prev(&mut self);
    fn select(&mut self, idx: usize);
    fn unselect(&mut self);
    fn selected(&self) -> Option<usize>;
    fn selected_item(&self) -> Option<&T>;
}

#[derive(Debug, Default)]
pub struct SelectableList<T: Display> {
    pub items: Vec<T>,
    pub state: ListState,
}

impl<T: Display> Selectable<T> for SelectableList<T> {
    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn prev(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn select(&mut self, idx: usize) {
        if idx < self.items.len() {
            self.state.select(Some(idx))
        }
    }

    fn unselect(&mut self) {
        self.state.select(None)
    }

    fn selected(&self) -> Option<usize> {
        self.state.selected()
    }

    fn selected_item(&self) -> Option<&T> {
        match self.state.selected() {
            Some(i) => Some(&self.items[i]),
            None => None,
        }
    }
}

impl<T: Display> Component for SelectableList<T> {
    fn render<B: Backend>(&mut self, frame: &mut Frame<'_, B>, area: Rect) {
        let start_menu = List::new(to_list_item(&self.items))
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::ITALIC)
                    .fg(Color::Cyan),
            )
            .highlight_symbol(">>");
        frame.render_stateful_widget(start_menu, area, &mut self.state)
    }
}

impl<T: Display> SelectableList<T> {
    pub fn with_items(items: Vec<T>) -> SelectableList<T> {
        let mut state = ListState::default();
        // If items is not empty, then select the first item
        state.select(if items.len() > 0 { Some(0) } else { None });
        Self { state, items }
    }
}

pub fn to_list_item<T: Display>(list: &Vec<T>) -> Vec<ListItem> {
    list.iter().map(|i| ListItem::new(i.to_string())).collect()
}
