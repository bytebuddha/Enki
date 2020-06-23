use crossterm::event::{Event, KeyCode};

use super::View;
use crate::actions::{Action, CursorAction, ViewAction};
use crate::core::EventHandler;

#[async_trait::async_trait]
impl EventHandler for View {
    async fn handle_event(&mut self, event: Event) -> crate::core::Result<Option<Action>> {
        match event {
            Event::Mouse(_) => Ok(None),
            Event::Resize(_x, _y) => Ok(None),
            Event::Key(key) => {
                match key.code {
                    KeyCode::Char(chr) => Ok(Some(Action::View(
                        Some(self.textarea.id),
                        ViewAction::Insert(chr.to_string()),
                    ))),
                    KeyCode::Backspace => Ok(Some(Action::View(
                        Some(self.textarea.id),
                        ViewAction::Cursor(CursorAction::Backspace),
                    ))),
                    KeyCode::Enter => Ok(Some(Action::View(
                        Some(self.textarea.id),
                        ViewAction::Enter,
                    ))),
                    KeyCode::Left => Ok(Some(Action::View(
                        Some(self.textarea.id),
                        ViewAction::Cursor(CursorAction::Left),
                    ))),
                    KeyCode::Right => Ok(Some(Action::View(
                        Some(self.textarea.id),
                        ViewAction::Cursor(CursorAction::Right),
                    ))),
                    KeyCode::Up => Ok(Some(Action::View(
                        Some(self.textarea.id),
                        ViewAction::Cursor(CursorAction::Up),
                    ))),
                    KeyCode::Down => Ok(Some(Action::View(
                        Some(self.textarea.id),
                        ViewAction::Cursor(CursorAction::Down),
                    ))),
                    KeyCode::Home => Ok(Some(Action::View(
                        Some(self.textarea.id),
                        ViewAction::Cursor(CursorAction::Home),
                    ))),
                    KeyCode::End => Ok(Some(Action::View(
                        Some(self.textarea.id),
                        ViewAction::Cursor(CursorAction::End),
                    ))),
                    KeyCode::PageUp => Ok(Some(Action::View(
                        Some(self.textarea.id),
                        ViewAction::Cursor(CursorAction::PageUp),
                    ))),
                    KeyCode::PageDown => Ok(Some(Action::View(
                        Some(self.textarea.id),
                        ViewAction::Cursor(CursorAction::PageDown),
                    ))),
                    KeyCode::Tab => Ok(Some(Action::View(Some(self.textarea.id), ViewAction::Tab))),
                    KeyCode::BackTab => Ok(Some(Action::View(
                        Some(self.textarea.id),
                        ViewAction::BackTab,
                    ))),
                    KeyCode::Delete => Ok(Some(Action::View(
                        Some(self.textarea.id),
                        ViewAction::Cursor(CursorAction::Delete),
                    ))),
                    //                    KeyCode::Insert => Ok(Some(Action::View(Some(self.id), ViewAction::Cursor(CursorAction::Insert)))),
                    //                    KeyCode::F(f) => Ok(Some(Action::View(Some(self.id), ViewAction::F(f)))),
                    //                    KeyCode::Null => Ok(Some(Action::View(Some(self.id), ViewAction::Null))),
                    //                    KeyCode::Esc  => Ok(Some(Action::View(Some(self.id), ViewAction::Esc)),
                    _ => Ok(None),
                }
            }
        }
    }
}
