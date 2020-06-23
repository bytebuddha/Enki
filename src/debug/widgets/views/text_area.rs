use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, List, Text, Widget};

use crate::view::View;

pub struct TextAreaDebugWidget<'a> {
    pub view: &'a View,
}

impl<'a> TextAreaDebugWidget<'a> {
    pub fn new(view: &'a View) -> TextAreaDebugWidget<'a> {
        TextAreaDebugWidget { view }
    }
}

impl<'a> Widget for TextAreaDebugWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Text Area ")
            .border_style(Style::default().fg(Color::Blue))
            .title_style(Style::default().fg(Color::LightBlue));
        let inner = block.inner(area);
        block.render(area, buf);
        let items = vec![
            Text::raw(format!("ViewPort Offset: {}", self.view.textarea.offset)),
            Text::raw(format!(
                "Invalid Before: {}",
                self.view.textarea.cache.n_before
            )),
            Text::raw(format!(
                "Line Count: {}",
                self.view.textarea.cache.lines.len()
            )),
            Text::raw(format!(
                "Invalid After: {}",
                self.view.textarea.cache.n_after
            )),
            Text::raw(format!(
                "Total: {}",
                self.view.textarea.cache.n_after
                    + self.view.textarea.cache.lines.len() as u64
                    + self.view.textarea.cache.n_after
            )),
            Text::raw(format!("Cursor Line: {}", self.view.textarea.cursor.1)),
            Text::raw(format!("Cursor Column: {}", self.view.textarea.cursor.0)),
        ];
        List::new(items.into_iter()).render(inner, buf);
    }
}
