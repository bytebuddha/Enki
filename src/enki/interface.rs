use crate::xi::{Client, ScrollTo};
use clap::App;
use crossterm::event::Event;
use log::{debug, trace};

use crate::actions::{Action, UiAction};
use crate::core::{Error as EnkiError, EventHandler, LoopResponse};
use crate::debug::{Debug, DebugWidgetBuilder};
use crate::editor::{Editor, EditorWidgetBuilder};
use crate::enki::{Context, Terminal};
use crate::prompt::PromptWidgetBuilder;
use crate::prompt::{Message as PromptMessage, Prompt, PromptResponse};

#[derive(Default)]
pub struct UserInterface {
    pub debug_showing: bool,
    pub debug: Debug,
    pub editor: Editor,
    pub prompt: Option<Prompt>,
}

impl UserInterface {
    pub fn render<'a, 'b, 'c>(
        &mut self,
        context: Context<'a, 'b, 'c>,
        term: &mut Terminal,
    ) -> crate::core::Result<()> {
        debug!("Beginning Render");
        let mut prompt_rect = None;
        term.backend.draw(|f| {
            if self.debug_showing {
                trace!("Rendering Debug Widget");
                f.render_stateful_widget(
                    DebugWidgetBuilder::default()
                        .editor(&self.editor)
                        .reactor(&context.reactor)
                        .config(&context.config)
                        .build()
                        .expect("Failed to build Debug Widget"),
                    f.size(),
                    &mut self.debug,
                );
            } else {
                trace!("Rendering Editor Widget");
                let editor = EditorWidgetBuilder::default()
                    .config(&context.config)
                    .build()
                    .expect("Failed to build Editor Widget");
                f.render_stateful_widget(editor, f.size(), &mut self.editor);
                if self.prompt.is_none() {
                    if let Some(view) = self.editor.views.get_current() {
                        f.set_cursor(view.textarea.cursor.0, view.textarea.cursor.1);
                    }
                }
            }
            if let Some(prompt) = &self.prompt {
                trace!("Rendering Prompt Widget");
                let prompt_widget = PromptWidgetBuilder::default()
                    .prompt(&prompt)
                    .config(&context.config)
                    .build()
                    .expect("failed to build prompt widget");
                f.render_widget(prompt_widget, f.size());
                prompt_rect = Some(f.size());
                let column: u16 = prompt
                    .chars
                    .chars()
                    .take(prompt.dex)
                    .fold(0, |acc, c| acc + translate_char_width(acc, c));
                f.set_cursor(column + 2, f.size().height);
            }
        })?;
        Ok(())
    }

    pub async fn handle_resize(
        &mut self,
        client: &mut Client,
        x: u16,
        y: u16,
    ) -> crate::core::Result<()> {
        trace!("Resizing Windoe: X({}), Y({})", x, y);
        self.editor.handle_resize(client, x as u64, y as u64).await
    }

    pub fn handle_action(
        &mut self,
        _context: Context,
        action: UiAction,
    ) -> crate::core::Result<LoopResponse> {
        match action {
            UiAction::ShowDebugWidget => self.debug_showing = true,
            UiAction::HideDebugWidget => self.debug_showing = false,
            UiAction::ToggleDebugWidget => self.debug_showing = !self.debug_showing,
            UiAction::ShowPrompt => self.prompt = Some(Prompt::default()),
            UiAction::HidePrompt => self.prompt = None,
            UiAction::TogglePrompt => {
                if self.prompt.is_none() {
                    self.prompt = Some(Prompt::default());
                } else {
                    self.prompt = None;
                }
            }
            _ => {}
        }
        Ok(LoopResponse::Render)
    }

    pub async fn handle_event<'b>(
        &mut self,
        app: &mut App<'b, 'b>,
        event: Event,
    ) -> crate::core::Result<Option<Action>> {
        if let Some(prompt) = &mut self.prompt {
            match prompt.handle_event(app, event) {
                PromptResponse::Continue => Ok(Some(Action::Ui(UiAction::Render))),
                PromptResponse::Cancel => {
                    self.prompt = None;
                    Ok(Some(Action::Ui(UiAction::Render)))
                }
                PromptResponse::Action(action) => {
                    self.prompt = None;
                    Ok(Some(action))
                }
                PromptResponse::Message(msg) => {
                    prompt.set_message(msg);
                    Ok(Some(Action::Ui(UiAction::Render)))
                }
            }
        } else if self.debug_showing {
            Ok(self.debug.handle_event(event).await?)
        } else {
            Ok(self.editor.handle_event(event).await?)
        }
    }

    pub async fn scroll_to(
        &mut self,
        scroll: ScrollTo,
        term: &mut Terminal,
    ) -> crate::core::Result<LoopResponse> {
        if let Some(view) = self.editor.views.get_mut(&scroll.view_id) {
            view.textarea.scroll_to(scroll, term).await
        } else {
            Err(EnkiError::UnknownView(scroll.view_id))
        }
    }

    pub fn set_prompt_message(&mut self, data: PromptMessage) {
        if let Some(prompt) = &mut self.prompt {
            prompt.set_message(data);
        } else {
            let mut prompt = Prompt::default();
            prompt.set_message(data);
            self.prompt = Some(prompt);
        }
    }
}

fn translate_char_width(position: u16, c: char) -> u16 {
    match c {
        // Caret notation means non-tab control characters are two columns wide
        '\x00'..='\x08' | '\x0a'..='\x1f' | '\x7f' => 2,
        '\t' => 4 - (position % 4),
        _ => 1,
    }
}
