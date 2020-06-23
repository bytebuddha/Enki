mod editor;
mod event;
mod shell;
mod ui;

use enki::actions::{get_arguments, Action, ParseActionError};

fn parse_action(input: &str) -> Result<Action, ParseActionError> {
    use enki::actions::parse_action as _parse_action;

    let mut app = get_arguments();
    _parse_action(&mut app, input)
}
