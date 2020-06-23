use derive_builder::Builder;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, List, ListState, StatefulWidget, Text};

use crate::editor::ViewList;

#[derive(Builder)]
pub struct ViewListWidget<'a> {
    pub views: &'a ViewList,
}

impl<'a> StatefulWidget for ViewListWidget<'a> {
    type State = ListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let iter = self
            .views
            .get_all()
            .map(|item| Text::raw(format!("{:?}", item.textarea.id)));
        List::new(iter)
            .highlight_symbol(">>")
            .block(
                Block::default()
                    .title(" View List ")
                    .borders(Borders::ALL)
                    .title_style(Style::default().fg(Color::LightRed))
                    .border_style(Style::default().fg(Color::Red)),
            )
            .render(area, buf, state);
    }
}
