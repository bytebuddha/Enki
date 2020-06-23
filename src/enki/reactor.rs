use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent};

use std::collections::HashMap;

use crate::actions::{Action, CursorAction, UiAction, ViewAction};

/// Struct used to map Events into Actions.
pub struct ActionReactor {
    data: HashMap<Event, Vec<Action>>,
}

impl ActionReactor {
    /// Create en empty ActionReactor.
    pub fn new() -> ActionReactor {
        ActionReactor {
            data: HashMap::new(),
        }
    }

    pub fn bindings(&self) -> Vec<(&Event, &Vec<Action>)> {
        self.data.iter().collect()
    }

    /// Bind a new event with the given action
    pub fn bind(&mut self, ev: Event, action: Vec<Action>) {
        let ev = self.sanitize_event(&ev);
        self.data.insert(ev, action);
    }

    /// Unbind any actions bound to the given event.
    pub fn unbind(&mut self, ev: Event) {
        let ev = self.sanitize_event(&ev);
        self.data.remove(&ev);
    }

    /// Return an action if one is bound to the event `ev`.
    pub fn react(&self, ev: &Event) -> Option<Vec<Action>> {
        let ev = self.sanitize_event(ev);
        self.data.get(&ev).map(Clone::clone)
    }

    /// This function will set all integer values in mouse events to 0.
    fn sanitize_event(&self, ev: &Event) -> Event {
        match ev {
            Event::Resize(_, _) => Event::Resize(0, 0),
            Event::Key(key_ev) => Event::Key(*key_ev),
            Event::Mouse(mouse) => match mouse {
                MouseEvent::ScrollDown(_, _, modifiers) => {
                    Event::Mouse(MouseEvent::ScrollDown(0, 0, *modifiers))
                }
                MouseEvent::ScrollUp(_, _, modifiers) => {
                    Event::Mouse(MouseEvent::ScrollUp(0, 0, *modifiers))
                }
                MouseEvent::Drag(btn, _, _, modifiers) => {
                    Event::Mouse(MouseEvent::Drag(*btn, 0, 0, *modifiers))
                }
                MouseEvent::Up(btn, _, _, modifiers) => {
                    Event::Mouse(MouseEvent::Up(*btn, 0, 0, *modifiers))
                }
                MouseEvent::Down(btn, _, _, modifiers) => {
                    Event::Mouse(MouseEvent::Down(*btn, 0, 0, *modifiers))
                }
            },
        }
    }
}

impl Default for ActionReactor {
    fn default() -> ActionReactor {
        let mut reactor = ActionReactor::new();
        let code = KeyCode::Char('c');
        let event = Event::Key(KeyEvent {
            code,
            modifiers: KeyModifiers::CONTROL,
        });
        reactor.bind(event, vec![Action::Quit]);

        let code = KeyCode::Char('p');
        let event = Event::Key(KeyEvent {
            code,
            modifiers: KeyModifiers::CONTROL,
        });
        reactor.bind(event, vec![Action::Ui(UiAction::TogglePrompt)]);

        let code = KeyCode::Left;
        let event = Event::Key(KeyEvent {
            code,
            modifiers: KeyModifiers::SHIFT,
        });
        reactor.bind(
            event,
            vec![Action::View(
                None,
                ViewAction::Cursor(CursorAction::LeftWord),
            )],
        );

        let code = KeyCode::Right;
        let event = Event::Key(KeyEvent {
            code,
            modifiers: KeyModifiers::SHIFT,
        });
        reactor.bind(
            event,
            vec![Action::View(
                None,
                ViewAction::Cursor(CursorAction::RightWord),
            )],
        );

        let code = KeyCode::F(12);
        let event = Event::Key(KeyEvent {
            code,
            modifiers: KeyModifiers::empty(),
        });
        reactor.bind(event, vec![Action::Ui(UiAction::ToggleDebugWidget)]);

        let code = KeyCode::Char('z');
        let event = Event::Key(KeyEvent {
            code,
            modifiers: KeyModifiers::CONTROL,
        });
        reactor.bind(event, vec![Action::View(None, ViewAction::Undo)]);

        let code = KeyCode::Char('z');
        let event = Event::Key(KeyEvent {
            code,
            modifiers: KeyModifiers::CONTROL | KeyModifiers::SHIFT,
        });
        reactor.bind(event, vec![Action::View(None, ViewAction::Redo)]);

        reactor
    }
}
