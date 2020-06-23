use clap::App;
use crossterm::event::{Event, KeyCode};

use super::{Prompt, PromptResponse};

impl Prompt {
    pub fn handle_event<'a>(&mut self, app: &mut App<'a, 'a>, event: Event) -> PromptResponse {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Enter => self.finalize(app),
                KeyCode::Backspace => self.back(),
                KeyCode::Delete => self.delete(),
                KeyCode::Left => self.left(),
                KeyCode::Right => self.right(),
                KeyCode::Char(c) => self.new_key(c),
                _ => PromptResponse::Continue,
            },
            _ => PromptResponse::Continue,
        }
    }
}
