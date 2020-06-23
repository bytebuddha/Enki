use super::View;
use crate::xi::ThemeSettings;
use derive_builder::Builder;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::widgets::Widget;

#[derive(Builder)]
pub struct Gutter<'a, 'b> {
    view: &'a View,
    start: Option<u64>,
    theme: Option<&'b ThemeSettings>,
}

impl<'a, 'b> Gutter<'a, 'b> {
    fn get_style(&self) -> Style {
        let mut style = Style::default();
        if let Some(theme) = self.theme {
            if let Some(c) = theme.gutter_foreground {
                style.fg = Color::Rgb(c.r, c.g, c.b);
            } else if let Some(c) = theme.foreground {
                style.fg = Color::Rgb(c.r, c.g, c.b);
            }
        }
        style.bg = self.background_color();
        style
    }

    fn background_color(&self) -> Color {
        let mut style = Style::default();
        if let Some(theme) = self.theme {
            if let Some(c) = theme.gutter {
                style.bg = Color::Rgb(c.r, c.g, c.b);
            } else if let Some(c) = theme.background {
                style.bg = Color::Rgb(c.r, c.g, c.b);
            }
        }
        style.bg
    }
}

impl<'a, 'b> Widget for Gutter<'a, 'b> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_background(area, self.background_color());
        for (dex, line_number) in self
            .view
            .textarea
            .cache
            .lines
            .iter()
            .skip(self.start.unwrap_or(0) as usize)
            .take(area.height as usize)
            .map(|item| {
                item.as_ref()
                    .and_then(|item| item.line_num.map(|item| format!("{}", item)))
            })
            .enumerate()
            .collect::<Vec<(usize, Option<String>)>>()
        {
            if let Some(line_no) = line_number {
                if line_no.len() == 1 {
                    buf.set_stringn(3, area.y + dex as u16, line_no, 3, self.get_style());
                } else if line_no.len() == 2 {
                    buf.set_stringn(2, area.y + dex as u16, line_no, 3, self.get_style());
                } else if line_no.len() == 3 {
                    buf.set_stringn(1, area.y + dex as u16, line_no, 3, self.get_style());
                } else {
                    buf.set_stringn(
                        1,
                        area.y + dex as u16,
                        &line_no[line_no.len() - 3..],
                        3,
                        self.get_style(),
                    );
                }
            }
        }
    }
}
