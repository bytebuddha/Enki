use derive_builder::Builder;
use tui::buffer::Buffer;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::widgets::{Block, Widget};

use super::{
    ConfigWidgetBuilder, PluginsWidgetBuilder, TextAreaDebugWidget, ViewDetailsWidgetBuilder,
};
use crate::view::View;

#[derive(Builder)]
pub struct ViewDebugWidget<'a> {
    pub view: &'a View,
}

impl<'a> Widget for ViewDebugWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default();
        let inner = block.inner(area);
        block.render(area, buf);
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                    Constraint::Max(15),
                ]
                .as_ref(),
            )
            .split(inner);
        let top_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(chunks[0]);
        let bottom_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(chunks[1]);
        TextAreaDebugWidget::new(&self.view).render(top_chunks[1], buf);
        ViewDetailsWidgetBuilder::default()
            .view(&self.view)
            .build()
            .expect("Failed to build View Details Widget")
            .render(top_chunks[0], buf);
        PluginsWidgetBuilder::default()
            .plugins(&self.view.plugins)
            .build()
            .expect("Failed to build Plugins Widget")
            .render(bottom_chunks[0], buf);
        ConfigWidgetBuilder::default()
            .config(self.view.textarea.config.as_ref())
            .build()
            .expect("Failed to build Config Widget")
            .render(bottom_chunks[1], buf);
    }
}
