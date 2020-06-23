use crossterm::event::{Event, KeyCode};
use tui::widgets::ListState;

use crate::actions::{Action, UiAction};

mod actions;
pub use self::actions::Actions;

mod widgets;
pub use self::widgets::DebugWidgetBuilder;

pub struct Debug {
    pub active_tab: usize,
    pub actions: Actions,
    pub view_list_state: ListState,
}

impl Debug {
    pub fn prev(&mut self) {
        if self.active_tab >= 2 {
            self.active_tab = 0;
        } else {
            self.active_tab += 1;
        }
    }

    pub fn next(&mut self) {
        if self.active_tab == 0 {
            self.active_tab = 2;
        } else {
            self.active_tab -= 1;
        }
    }

    pub async fn handle_event(&mut self, event: Event) -> crate::Result<Option<Action>> {
        match event {
            Event::Key(key) => {
                match key.code {
                    KeyCode::Left => self.next(),
                    KeyCode::Right => self.prev(),
                    _ => return Ok(None),
                }
                Ok(Some(Action::Ui(UiAction::Render)))
            }
            _ => Ok(None),
        }
    }
}

impl Default for Debug {
    fn default() -> Debug {
        let mut view_list_state = ListState::default();
        view_list_state.select(Some(0));
        Debug {
            active_tab: 0,
            actions: Actions::default(),
            view_list_state,
        }
    }
}
