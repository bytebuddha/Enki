use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use enki::actions::{Action, UiAction};
use enki::enki::ActionReactor;

#[test]
fn react() {
    let mut reactor = ActionReactor::new();

    let q_code = KeyCode::Char('q');
    let modifiers = KeyModifiers::CONTROL;
    reactor.bind(
        Event::Key(KeyEvent {
            code: q_code.clone(),
            modifiers,
        }),
        vec![Action::Quit],
    );
    assert_eq!(
        Some(vec![Action::Quit]),
        reactor.react(&Event::Key(KeyEvent {
            code: q_code,
            modifiers
        }))
    );

    let p_code = KeyCode::Char('p');
    reactor.bind(
        Event::Key(KeyEvent {
            code: p_code.clone(),
            modifiers,
        }),
        vec![Action::Ui(UiAction::ToggleDebugWidget)],
    );
    let modifiers = KeyModifiers::CONTROL;
    assert_eq!(
        Some(vec![Action::Ui(UiAction::ToggleDebugWidget)]),
        reactor.react(&Event::Key(KeyEvent {
            code: p_code,
            modifiers: modifiers.clone()
        }))
    );
}
