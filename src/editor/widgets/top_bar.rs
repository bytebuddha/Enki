use derive_builder::Builder;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::Color;
use tui::style::Style;
use tui::widgets::Widget;

use super::Editor;

#[derive(Builder)]
pub struct TopBar<'a> {
    editor: &'a Editor,
}

impl<'a> TopBar<'a> {
    fn get_background_style(&self) -> Color {
        Color::DarkGray
    }

    fn get_tab_style(&self) -> Style {
        Style::default().bg(Color::DarkGray).fg(Color::Gray)
    }
}

impl<'a> Widget for TopBar<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_background(area, self.get_background_style());

        let tabs = "•".repeat(self.editor.views.len());
        let x = (area.x + area.width - tabs.len() as u16) / 2;
        buf.set_string(x, area.y, &tabs, self.get_tab_style());
        let view_index = if let Some(index) = self.editor.views.get_current_index() {
            x + index as u16
        } else {
            x
        };
        buf.get_mut(view_index, area.y).set_fg(Color::White);
    }
}
