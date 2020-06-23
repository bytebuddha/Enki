use crate::enki::Configuration;
use crate::textarea::TextArea;
use crate::xi::{StyleCache, ThemeSettings};
use derive_builder::Builder;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::Color;
use tui::widgets::StatefulWidget;
use tui::widgets::Widget;

mod line;
pub use self::line::{LineWidget, LineWidgetBuilder};

mod chunk;
pub use self::chunk::{ChunkWidget, ChunkWidgetBuilder};

#[derive(Builder)]
pub struct TextAreaWidget<'a, 'b, 'c> {
    #[builder(default)]
    pub theme: Option<&'a ThemeSettings>,
    #[builder(default)]
    pub styles: Option<&'b StyleCache>,
    pub global_config: &'c Configuration,
}

impl<'a, 'b, 'c> StatefulWidget for TextAreaWidget<'a, 'b, 'c> {
    type State = TextArea;

    fn render(self, area: Rect, buf: &mut Buffer, view: &mut Self::State) {
        if let Some(theme) = self.theme {
            if let Some(color) = theme.background {
                buf.set_background(area, Color::Rgb(color.r, color.g, color.b));
            }
        }
        let lines = view.cache.lines.iter().skip(view.offset as usize);

        for (line_index, line) in lines.enumerate() {
            let start_y = area.y + line_index as u16;
            let start_x = area.x;
            if start_y < area.height + area.y && start_x < area.width + area.x {
                let line_rect = Rect {
                    x: start_x,
                    y: start_y,
                    width: area.width,
                    height: 1,
                };
                LineWidgetBuilder::default()
                    .styles(self.styles)
                    .line(line.as_ref())
                    .theme(self.theme)
                    .global_config(&self.global_config)
                    .config(view.config.as_ref())
                    .build()
                    .expect("Failed to build Line Widget")
                    .render(line_rect, buf);
            }
        }
        view.area = Some(area);
    }
}
