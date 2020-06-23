use derive_builder::Builder;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Widget};

use crate::prompt::Message;

#[derive(Builder)]
pub struct MessageBox<'a> {
    msg: &'a Message,
    #[builder(setter(strip_option))]
    wrapped_lines: Option<Vec<String>>,
}

impl<'a> MessageBox<'a> {
    fn get_border_style(&self) -> Style {
        Style::default()
            .fg(self.msg.border_fg)
            .bg(self.msg.border_bg)
    }

    fn get_title_style(&self) -> Style {
        Style::default().fg(self.msg.title_fg).bg(self.msg.title_bg)
    }

    fn get_msg_style(&self) -> Style {
        Style::default().fg(Color::White).bg(Color::DarkGray)
    }

    fn get_block(&self) -> Block {
        Block::default()
            .borders(Borders::ALL)
            .title_style(self.get_title_style())
            .border_style(self.get_border_style())
    }
}

impl<'a> Widget for MessageBox<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_background(area, Color::DarkGray);
        let mut block = self.get_block();
        if let Some(header) = &self.msg.title {
            block = block.title(&header);
        }
        block.render(area, buf);
        let data_rect = block.inner(area);
        buf.set_background(data_rect, Color::DarkGray);
        let msg_style = self.get_msg_style();
        let msg_text = self.msg.text.clone();
        let string = self.wrapped_lines.unwrap_or_else(|| {
            msg_text
                .clone()
                .split(' ')
                .map(ToString::to_string)
                .collect()
        });
        for (line_index, line) in string.iter().enumerate() {
            buf.set_stringn(
                data_rect.x,
                data_rect.y + line_index as u16,
                line,
                line.len(),
                msg_style,
            );

            let remains = (data_rect.x + data_rect.width) - line.len() as u16;
            let y = data_rect.y + line_index as u16;
            let x = data_rect.x + line.len() as u16;
            buf.set_stringn(
                x,
                y,
                &" ".repeat(remains as usize),
                remains as usize - 1,
                msg_style,
            );
        }
    }
}
