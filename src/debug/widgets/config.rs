use derive_builder::Builder;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, List, Text, Widget};

use crate::enki::Configuration;

#[derive(Builder)]
pub struct ConfigurationWidget<'a> {
    pub config: &'a Configuration,
}

impl<'a> Widget for ConfigurationWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title_style(Style::default().fg(Color::Green))
            .border_style(Style::default().fg(Color::LightGreen))
            .title(" Configuration Settings ");
        block.render(area, buf);
        let inner = block.inner(area);
        let vars = self.config.vars();
        let iter = vars
            .iter()
            .map(|item| Text::raw(format!("{:?} = {:?}", item.0, item.1)));
        List::new(iter).render(inner, buf);
    }
}
