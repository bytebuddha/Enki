use derive_builder::Builder;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, List, Text, Widget};

use crate::view::View;

#[derive(Builder)]
pub struct ViewDetailsWidget<'a> {
    pub view: &'a View,
}

impl<'a> Widget for ViewDetailsWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title(" Details ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray))
            .title_style(Style::default().fg(Color::Gray));
        let inner = block.inner(area);
        block.render(area, buf);

        let mut items = vec![Text::raw(format!("Language: {:?}", self.view.language))];
        if let Some(area) = self.view.area {
            items.push(Text::raw(format!(
                "Area: ({}, {}, {}, {})",
                area.x, area.y, area.width, area.height
            )));
        }
        List::new(items.into_iter()).render(inner, buf);
    }
}
