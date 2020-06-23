use enki::actions::{Action, UiAction};

use super::parse_action;

fn test_action(s: &str, action: UiAction) {
    assert_eq!(Ok(Action::Ui(action)), parse_action(s));
}

#[test]
fn parse() {
    test_action("ui debug", UiAction::ToggleDebugWidget);
    test_action("ui debug -s", UiAction::ShowDebugWidget);
    test_action("ui debug -r", UiAction::HideDebugWidget);
    test_action("ui prompt", UiAction::TogglePrompt);
    test_action("ui prompt -s", UiAction::ShowPrompt);
    test_action("ui prompt -r", UiAction::HidePrompt);
}
