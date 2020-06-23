use enki::actions::{Action, EditorAction};

use super::parse_action;

fn test_action(s: &str, action: EditorAction) {
    assert_eq!(Ok(Action::Editor(action)), parse_action(s));
}

#[test]
fn parse() {
    test_action("editor themes", EditorAction::ListThemes);
    test_action(
        "editor themes --set themename",
        EditorAction::SetTheme("themename".into()),
    );
    test_action("editor languages", EditorAction::ListLanguages);
    test_action("editor views", EditorAction::ListViews);
    test_action("editor views --next", EditorAction::NextView);
    test_action("editor views --previous", EditorAction::PreviousView);
    test_action("editor open", EditorAction::Open(None));
    test_action(
        "editor open -f file.txt",
        EditorAction::Open(Some("file.txt".into())),
    );
}
