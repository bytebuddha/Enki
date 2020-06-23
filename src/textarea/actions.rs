use super::TextArea;
use crate::actions::{CursorAction, FindAction, ViewAction};
use crate::core::{ActionHandler, LoopResponse};
use crate::enki::Context;
use crate::xi::client::ClientExt;
use log::error;

#[async_trait::async_trait]
impl ActionHandler for TextArea {
    type Event = ViewAction;

    async fn handle_action<'a, 'b, 'c>(
        &mut self,
        context: Context<'a, 'b, 'c>,
        action: Self::Event,
    ) -> crate::core::Result<LoopResponse> {
        match action {
            ViewAction::Cursor(CursorAction::Up) => {
                context.client.simple_edit(self.id, "move_up").await?
            }
            ViewAction::Cursor(CursorAction::Down) => {
                context.client.simple_edit(self.id, "move_down").await?
            }
            ViewAction::Cursor(CursorAction::Left) => {
                context.client.simple_edit(self.id, "move_left").await?
            }
            ViewAction::Cursor(CursorAction::LeftWord) => {
                context
                    .client
                    .simple_edit(self.id, "move_word_left")
                    .await?
            }
            ViewAction::Cursor(CursorAction::Right) => {
                context.client.simple_edit(self.id, "move_right").await?
            }
            ViewAction::Cursor(CursorAction::RightWord) => {
                context
                    .client
                    .simple_edit(self.id, "move_word_right")
                    .await?
            }
            ViewAction::Cursor(CursorAction::PageUp) => {
                context
                    .client
                    .simple_edit(self.id, "scroll_page_up")
                    .await?
            }
            ViewAction::Cursor(CursorAction::PageDown) => {
                context
                    .client
                    .simple_edit(self.id, "scroll_page_down")
                    .await?
            }
            ViewAction::Cursor(CursorAction::Home) => {
                context
                    .client
                    .simple_edit(self.id, "move_to_left_end_of_line")
                    .await?
            }
            ViewAction::Cursor(CursorAction::End) => {
                context
                    .client
                    .simple_edit(self.id, "move_to_right_end_of_line")
                    .await?
            }
            ViewAction::Cursor(CursorAction::Backspace) => {
                context
                    .client
                    .simple_edit(self.id, "delete_backward")
                    .await?
            }
            ViewAction::Cursor(CursorAction::Delete) => {
                context
                    .client
                    .simple_edit(self.id, "delete_forward")
                    .await?
            }
            ViewAction::SetLanguage(lang) => context.client.set_language(self.id, &lang).await?,
            ViewAction::Insert(data) => context.client.insert(self.id, &data).await?,
            ViewAction::Enter => {
                context
                    .client
                    .simple_edit(self.id, "insert_newline")
                    .await?
            }
            ViewAction::Tab => context.client.simple_edit(self.id, "insert_tab").await?,
            ViewAction::Undo => context.client.simple_edit(self.id, "undo").await?,
            ViewAction::Redo => context.client.simple_edit(self.id, "redo").await?,
            ViewAction::Save(file) => {
                if let Some(file) = file {
                    context.client.save(self.id, &file).await?
                } else {
                    error!("Saving without specify a file name is unimplemented");
                    unimplemented!()
                }
            }
            ViewAction::Find(action) => match action {
                FindAction::Query(query, case, regex, words) => {
                    context
                        .client
                        .find(self.id, &query, case, regex, words)
                        .await?;
                    context.client.highlight_find(self.id, true).await?
                }
                FindAction::Previous(_, _) => unimplemented!(),
                FindAction::Next(_, _) => unimplemented!(),
            },
            ViewAction::BackTab => {}
        }
        Ok(LoopResponse::Continue)
    }
}
