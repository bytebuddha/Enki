use crate::core::consts::PROMPT_CHARS;
use crate::enki::Configuration;
use derive_builder::Builder;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::widgets::Widget;

#[derive(Builder)]
pub struct InputWidget<'a, 'b> {
    chars: &'a str,
    config: &'b Configuration,
}

impl<'a, 'b> InputWidget<'a, 'b> {
    fn get_prompt_style(&self) -> Style {
        let mut style = Style::default();
        style.fg = Color::White;
        style.bg = Color::DarkGray;
        style
    }
}

impl<'a, 'b> Widget for InputWidget<'a, 'b> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_background(area, Color::DarkGray);
        let prompt_chars = self
            .config
            .get_default_t("prompt_char", PROMPT_CHARS.to_string());
        let prompt_chars_len = prompt_chars.len() as u16;
        let cell = buf.get_mut(area.x, area.y);
        cell.style = self.get_prompt_style();
        cell.symbol = self
            .config
            .get_default_t("prompt_char", PROMPT_CHARS.to_string());

        let cell = buf.get_mut(area.x + 1, area.y);
        cell.style = self.get_prompt_style();
        cell.symbol = "".into();

        buf.set_string(
            area.x + prompt_chars_len,
            area.y,
            self.chars,
            self.get_prompt_style(),
        );

        for letter in area.x as usize + 3 + self.chars.len()..area.width as usize {
            let cell = buf.get_mut(letter as u16, area.y);
            cell.symbol = " ".into();
        }
    }
}
