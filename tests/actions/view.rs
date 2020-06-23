use enki::actions::{ Action, EditorAction, ViewAction, UiAction, CursorAction, FindAction };
use crate::xi::tabs::ViewId;

use super::parse_action;

fn test_action(s: &str, action: ViewAction) {
    assert_eq!(Ok(Action::View(id.map(ViewId::from), action)), parse_action(s));
}

#[test]
fn parse() {
    test_action("view -v 1 cursor up", ViewAction::Cursor(CursorAction::Up));
    test_action("view -v 1 cursor down", ViewAction::Cursor(CursorAction::Down));
    test_action("view -v 1 cursor left", ViewAction::Cursor(CursorAction::Left));
    test_action("view -v 1 cursor right", ViewAction::Cursor(CursorAction::Right));
    test_action("view -v 1 cursor pagedown", ViewAction::Cursor(CursorAction::PageDown));
    test_action("view -v 1 cursor pageup", ViewAction::Cursor(CursorAction::PageUp));
    test_action("view -v 1 cursor end", ViewAction::Cursor(CursorAction::End));
    test_action("view -v 1 cursor delete", ViewAction::Cursor(CursorAction::Delete));
    test_action("view -v 1 lang -l Rust", ViewAction::SetLanguage("Rust".into()));
    test_action("view -v 1 save -f file.txt", ViewAction::Save(Some("file.txt".into())));
    test_action("view -v 1 save", ViewAction::Save(None));
    test_action("view -v 1 find this", ViewAction::Find(FindAction::Query("this".into(), false, false, false)));
    test_action("view -v 1 find this --regex --case", ViewAction::Find(FindAction::Query("this".into(), true, true, false)));
}
