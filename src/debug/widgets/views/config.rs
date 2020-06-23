use crate::xi::ConfigChanges;
use derive_builder::Builder;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, List, Text, Widget};

#[derive(Builder)]
pub struct ConfigWidget<'a> {
    pub config: Option<&'a ConfigChanges>,
}

impl<'a> Widget for ConfigWidget<'a> {
    fn render(self, rect: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Configuration ")
            .border_style(Style::default().fg(Color::Cyan))
            .title_style(Style::default().fg(Color::LightCyan));
        let inner = block.inner(rect);
        block.render(rect, buf);
        if let Some(changes) = self.config {
            let changes = vec![
                Text::raw(format!("Font Face: {:?}", changes.font_face)),
                Text::raw(format!("Font Size: {:?}", changes.font_size)),
                Text::raw(format!("Line Ending: {:?}", changes.line_ending)),
                Text::raw(format!(
                    "Plugin Search Path: {:?}",
                    changes.plugin_search_path
                )),
                Text::raw(format!("Tab Size: {:?}", changes.tab_size)),
                Text::raw(format!(
                    "Tabs To Spaces: {:?}",
                    changes.translate_tabs_to_spaces
                )),
                Text::raw(format!("Word Wrap: {:?}", changes.word_wrap)),
            ];

            List::new(changes.into_iter()).render(inner, buf);
        }
    }
}
