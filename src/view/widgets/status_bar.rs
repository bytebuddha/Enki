use derive_builder::Builder;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::Color;
use tui::widgets::Widget;

#[derive(Builder)]
pub struct StatusBar {}

impl Widget for StatusBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_background(area, Color::LightCyan);
    }
}
