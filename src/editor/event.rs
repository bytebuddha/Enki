use crossterm::event::Event;
use log::error;

use super::Editor;
use crate::actions::Action;
use crate::core::EventHandler;

#[async_trait::async_trait]
impl EventHandler for Editor {
    async fn handle_event(&mut self, event: Event) -> crate::core::Result<Option<Action>> {
        if let Some(view) = self.views.get_current_mut() {
            view.handle_event(event).await
        } else {
            error!("No Current View is set in the editor!");
            Ok(None)
        }
    }
}
