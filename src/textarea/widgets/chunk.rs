use crate::core::consts::TAB_SIZE;
use crate::xi::ThemeSettings;
use derive_builder::Builder;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::widgets::Widget;

#[derive(Default, Builder)]
pub struct ChunkWidget<'a, 'b> {
    #[builder(default)]
    #[builder(setter(strip_option))]
    text: Option<&'a str>,
    #[builder(default)]
    background: Option<Color>,
    #[builder(default)]
    foreground: Option<Color>,
    #[builder(default)]
    italic: Option<bool>,
    #[builder(default)]
    underlined: Option<bool>,
    #[builder(default)]
    theme: Option<&'b ThemeSettings>,
    #[builder(default)]
    tab_size: Option<u16>,
}

impl<'a, 'b> Widget for ChunkWidget<'a, 'b> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.text.is_none() {
            if let Some(background) = self.background {
                buf.set_background(area, background);
            }
            return;
        }
        let text = self.text.unwrap();
        let mut data = String::new();
        // This Variable is undefined? WTF
        #[allow(unused_variables)]
        let mut position: u16 = 0;
        let tab_size = self.tab_size.unwrap_or(TAB_SIZE);
        for chr in text.chars() {
            match chr {
                '\x00'..='\x08' | '\x0a'..='\x1f' | '\x7f' => {
                    // Render in caret notation, i.e. '\x02' is rendered as '^B'
                    data.push('^');
                    data.push((chr as u8 ^ 0x40u8) as char);
                    position += 2;
                }
                '\t' => {
                    data.push_str(&" ".repeat(4));
                    position += tab_size;
                }
                _ => {
                    data.push(chr);
                    position += 1;
                }
            }
        }

        buf.set_stringn(area.x, area.y, &data, data.len(), get_style(&self));
    }
}

fn get_style<'a, 'b>(chunk: &ChunkWidget<'a, 'b>) -> Style {
    let mut style = Style::default();

    if let Some(background) = chunk.background {
        style.bg = background;
    } else if let Some(theme) = chunk.theme {
        if let Some(color) = theme.background {
            style.bg = Color::Rgb(color.r, color.g, color.b);
        }
    }

    if let Some(foreground) = chunk.foreground {
        style.fg = foreground;
    } else if let Some(theme) = chunk.theme {
        if let Some(color) = theme.foreground {
            style.fg = Color::Rgb(color.r, color.g, color.b);
        }
    }

    if let Some(true) = chunk.underlined {
        style.modifier.contains(Modifier::UNDERLINED);
    }

    if let Some(true) = chunk.italic {
        style.modifier.contains(Modifier::ITALIC);
    }

    style
}
