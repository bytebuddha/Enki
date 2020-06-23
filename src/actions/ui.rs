#[derive(Debug, PartialEq, Clone)]
pub enum UiAction {
    ShowPrompt,
    HidePrompt,
    TogglePrompt,
    ShowDebugWidget,
    HideDebugWidget,
    ToggleDebugWidget,
    Render,
}
