mod input;
pub use self::input::InputWidgetBuilder;

mod msg_box;
pub use self::msg_box::MessageBoxBuilder;

use crate::enki::Configuration;
use derive_builder::Builder;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::Widget;

use crate::prompt::Prompt;

#[derive(Builder)]
pub struct PromptWidget<'a, 'b> {
    prompt: &'a Prompt,
    config: &'b Configuration,
}

impl<'a, 'b> Widget for PromptWidget<'a, 'b> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if let Some(msg) = &self.prompt.message {
            //            let lines = msg.text.split("\n").collect::<Vec<&str>>();
            let lines = wrap_lines(&msg.text, area.width as usize - 2);
            let line_count = lines.len() as u16;
            let y = area.y + area.height - 3 - line_count;
            let msg_box_rect = Rect {
                x: area.x,
                y,
                width: area.width,
                height: line_count + 2,
            };
            MessageBoxBuilder::default()
                .msg(&msg)
                .wrapped_lines(lines)
                .build()
                .expect("Failed to build message box widget")
                .render(msg_box_rect, buf);
        }

        let input_rect = Rect {
            x: area.x,
            y: area.y + area.height - 1,
            width: area.width,
            height: 1,
        };
        InputWidgetBuilder::default()
            .chars(&self.prompt.chars)
            .config(&self.config)
            .build()
            .expect("Failed to build Prompt Input Widget")
            .render(input_rect, buf);
    }
}

fn wrap_lines(msg: &str, width: usize) -> Vec<String> {
    let mut lines = vec![];
    for original_line in msg.split('\n').collect::<Vec<&str>>() {
        for line in textwrap::wrap(original_line, width) {
            lines.push(line.into());
        }
    }
    lines
}
