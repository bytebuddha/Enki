use super::Editor;
use crate::actions::EditorAction;
use crate::core::{ActionHandler, LoopResponse};
use crate::enki::Context;
use crate::xi::ClientExt;

#[async_trait::async_trait]
impl ActionHandler for Editor {
    type Event = EditorAction;

    async fn handle_action<'a, 'b, 'c>(
        &mut self,
        context: Context<'a, 'b, 'c>,
        action: EditorAction,
    ) -> crate::Result<LoopResponse> {
        match action {
            EditorAction::NextView => {
                self.views.next();
                return Ok(LoopResponse::Render);
            }
            EditorAction::PreviousView => {
                self.views.prev();
                return Ok(LoopResponse::Render);
            }
            EditorAction::SetTheme(theme) => context.client.set_theme(&theme).await?,
            EditorAction::ListPlugins => unreachable!(),
            EditorAction::ListThemes => unreachable!(),
            EditorAction::ListViews => unreachable!(),
            EditorAction::ListLanguages => unreachable!(),
            EditorAction::Open(_) => unreachable!(),
        }
        Ok(LoopResponse::Continue)
    }
}
