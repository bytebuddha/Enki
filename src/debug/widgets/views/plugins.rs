use derive_builder::Builder;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, List, Text, Widget};

#[derive(Builder)]
pub struct PluginsWidget<'a> {
    plugins: &'a [String],
}

impl<'a> Widget for PluginsWidget<'a> {
    fn render(self, rect: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Plugins ")
            .border_style(Style::default().fg(Color::Magenta))
            .title_style(Style::default().fg(Color::LightMagenta));
        let inner = block.inner(rect);
        block.render(rect, buf);
        List::new(self.plugins.iter().map(|item| Text::raw(&*item))).render(inner, buf);
    }
}
