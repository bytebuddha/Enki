mod event;
pub use self::event::{parse_event, ParseEventError};
mod settings;
pub use self::settings::SettingsAction;

mod editor;
pub use self::editor::EditorAction;

mod view;
pub use self::view::{FindAction, ViewAction};

mod cursor;
pub use self::cursor::CursorAction;

mod args;
pub use self::args::get_arguments;

mod parse;
pub use self::parse::{parse_action, ParseActionError};

mod ui;
pub use self::ui::UiAction;

use crate::xi::ViewId;

#[derive(Debug, PartialEq, Clone)]
pub enum Action {
    Quit,
    ShellCommand(Vec<String>),
    Editor(EditorAction),
    Ui(UiAction),
    View(Option<ViewId>, ViewAction),
    Settings(SettingsAction),
}
