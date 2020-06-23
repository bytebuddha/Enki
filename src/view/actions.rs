use super::View;
use crate::actions::ViewAction;
use crate::core::ActionHandler;
use crate::core::LoopResponse;
use crate::enki::Context;

#[async_trait::async_trait]
impl ActionHandler for View {
    type Event = ViewAction;

    async fn handle_action<'a, 'b, 'c>(
        &mut self,
        context: Context<'a, 'b, 'c>,
        action: Self::Event,
    ) -> crate::core::Result<LoopResponse> {
        self.textarea.handle_action(context, action).await
    }
}
