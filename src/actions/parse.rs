use crate::xi::ViewId;
use clap::{App, ArgMatches, Error, ErrorKind};
use serde_json::{Error as JsonError, Value};

use std::num::ParseIntError;
use std::path::PathBuf;

use super::{
    parse_event, Action, CursorAction, EditorAction, FindAction, ParseEventError, SettingsAction,
    UiAction, ViewAction,
};

#[derive(Debug, PartialEq)]
pub enum ParseActionError {
    ExpectedArgument(String),
    UnknownArgument(String),
    Clap {
        message: String,
        kind: ErrorKind,
        info: Option<Vec<String>>,
    },
    Json(String),
    IntParse(ParseIntError),
    Event(ParseEventError),
}

impl From<Error> for ParseActionError {
    fn from(err: Error) -> ParseActionError {
        let Error {
            message,
            kind,
            info,
        } = err;
        ParseActionError::Clap {
            message,
            info,
            kind,
        }
    }
}

impl From<ParseEventError> for ParseActionError {
    fn from(err: ParseEventError) -> ParseActionError {
        ParseActionError::Event(err)
    }
}

impl From<JsonError> for ParseActionError {
    fn from(err: JsonError) -> ParseActionError {
        ParseActionError::Json(err.to_string())
    }
}

impl From<ParseIntError> for ParseActionError {
    fn from(err: ParseIntError) -> ParseActionError {
        ParseActionError::IntParse(err)
    }
}

pub fn parse_action<'a>(app: &mut App<'a, 'a>, input: &str) -> Result<Action, ParseActionError> {
    let matches = app.get_matches_from_safe_borrow(input.split(' '))?;

    // If this is a shell command parse as such and return early.
    if let Some(raw_values) = matches.values_of("cmd") {
        let values = raw_values.map(ToString::to_string);
        return Ok(Action::ShellCommand(values.collect()));
    }

    match matches.subcommand() {
        ("editor", Some(matches)) => parse_editor_action(matches),
        ("view", Some(matches)) => parse_view_action(matches),
        ("ui", Some(matches)) => parse_ui_action(matches),
        ("settings", Some(matches)) => parse_settings_action(app, matches),
        ("quit", _) => Ok(Action::Quit),
        (arg, _) => Err(ParseActionError::UnknownArgument(arg.to_string())),
    }
}

fn parse_settings_action<'a>(
    app: &mut App<'a, 'a>,
    matches: &ArgMatches<'a>,
) -> Result<Action, ParseActionError> {
    match matches.subcommand() {
        ("get", Some(matches)) => Ok(Action::Settings(SettingsAction::ConfigGet(
            matches.value_of("key").unwrap().into(),
        ))),
        ("set", Some(matches)) => {
            let key = matches.value_of("key").unwrap().into();
            let raw_value = matches.value_of("value").unwrap();
            let value: Value = serde_json::from_str(raw_value)?;
            Ok(Action::Settings(SettingsAction::ConfigBind(key, value)))
        }
        ("bind", Some(matches)) => {
            let raw_event = matches.value_of("event").unwrap();
            let raw_actions = matches.values_of("actions").unwrap();

            let mut final_actions = vec![];
            for raw_action in raw_actions.collect::<Vec<&str>>().join(" ").split(';') {
                final_actions.push(parse_action(app, &raw_action)?);
            }
            let event = parse_event(raw_event)?;
            Ok(Action::Settings(SettingsAction::EventBind(
                event,
                final_actions,
            )))
        }
        ("unbind", Some(matches)) => {
            let raw_event = matches.value_of("event").unwrap();
            Ok(Action::Settings(SettingsAction::EventUnbind(parse_event(
                raw_event,
            )?)))
        }
        (arg, _) => Err(ParseActionError::UnknownArgument(arg.to_string())),
    }
}

fn parse_editor_action<'a>(matches: &ArgMatches<'a>) -> Result<Action, ParseActionError> {
    match matches.subcommand() {
        ("open", None) => Ok(Action::Editor(EditorAction::Open(None))),
        ("open", Some(matches)) => {
            let value = matches.value_of("file_name").map(ToString::to_string);
            Ok(Action::Editor(EditorAction::Open(value)))
        }
        ("views", None) => Ok(Action::Editor(EditorAction::ListViews)),
        ("views", Some(matches)) => {
            if matches.is_present("next") {
                Ok(Action::Editor(EditorAction::NextView))
            } else if matches.is_present("previous") {
                Ok(Action::Editor(EditorAction::PreviousView))
            } else {
                Ok(Action::Editor(EditorAction::ListViews))
            }
        }
        ("languages", _) => Ok(Action::Editor(EditorAction::ListLanguages)),
        ("plugins", _) => Ok(Action::Editor(EditorAction::ListPlugins)),
        ("themes", None) => Ok(Action::Editor(EditorAction::ListThemes)),
        ("themes", Some(matches)) => {
            if let Some(theme) = matches.value_of("theme") {
                Ok(Action::Editor(EditorAction::SetTheme(theme.into())))
            } else {
                Ok(Action::Editor(EditorAction::ListThemes))
            }
        }
        (arg, _) => Err(ParseActionError::UnknownArgument(arg.to_string())),
    }
}

fn parse_view_action<'a>(matches: &ArgMatches<'a>) -> Result<Action, ParseActionError> {
    let view = if let Some(view) = matches.value_of("view") {
        let view_num: usize = view.parse()?;
        Some(ViewId::from(view_num))
    } else {
        None
    };
    match matches.subcommand() {
        ("cursor", Some(matches)) => parse_cursor_action(view, matches),
        ("save", None) => Ok(Action::View(view, ViewAction::Save(None))),
        ("insert", Some(matches)) => Ok(Action::View(
            view,
            ViewAction::Insert(matches.value_of("chars").unwrap().into()),
        )),
        ("save", Some(matches)) => {
            let file_name = matches.value_of("file_name").map(PathBuf::from);
            Ok(Action::View(view, ViewAction::Save(file_name)))
        }
        ("lang", Some(matches)) => {
            let language = matches
                .value_of("language")
                .map(ToString::to_string)
                .unwrap();
            Ok(Action::View(view, ViewAction::SetLanguage(language)))
        }
        ("find", Some(matches)) => {
            if let Some(query) = matches.value_of("query") {
                let regex = matches.is_present("regex");
                let case = matches.is_present("case");
                let words = matches.is_present("words");
                Ok(Action::View(
                    view,
                    ViewAction::Find(FindAction::Query(query.into(), case, regex, words)),
                ))
            } else if matches.is_present("next") {
                Ok(Action::View(
                    view,
                    ViewAction::Find(FindAction::Next(true, true)),
                ))
            } else if matches.is_present("previous") {
                Ok(Action::View(
                    view,
                    ViewAction::Find(FindAction::Previous(true, true)),
                ))
            } else {
                Err(ParseActionError::ExpectedArgument("query".into()))
            }
        }
        (arg, _) => Err(ParseActionError::UnknownArgument(arg.to_string())),
    }
}

fn parse_cursor_action<'a>(
    view: Option<ViewId>,
    matches: &ArgMatches<'a>,
) -> Result<Action, ParseActionError> {
    match matches.subcommand() {
        ("up", _) => Ok(Action::View(view, ViewAction::Cursor(CursorAction::Up))),
        ("down", _) => Ok(Action::View(view, ViewAction::Cursor(CursorAction::Down))),
        ("left", _) => Ok(Action::View(view, ViewAction::Cursor(CursorAction::Left))),
        ("right", _) => Ok(Action::View(view, ViewAction::Cursor(CursorAction::Right))),
        ("pageup", _) => Ok(Action::View(view, ViewAction::Cursor(CursorAction::PageUp))),
        ("pagedown", _) => Ok(Action::View(
            view,
            ViewAction::Cursor(CursorAction::PageDown),
        )),
        ("home", _) => Ok(Action::View(view, ViewAction::Cursor(CursorAction::Home))),
        ("end", _) => Ok(Action::View(view, ViewAction::Cursor(CursorAction::End))),
        ("backspace", _) => Ok(Action::View(
            view,
            ViewAction::Cursor(CursorAction::Backspace),
        )),
        ("delete", _) => Ok(Action::View(view, ViewAction::Cursor(CursorAction::Delete))),
        (arg, _) => Err(ParseActionError::UnknownArgument(arg.to_string())),
    }
}

fn parse_ui_action<'a>(matches: &ArgMatches<'a>) -> Result<Action, ParseActionError> {
    match matches.subcommand() {
        ("debug", None) => Ok(Action::Ui(UiAction::ToggleDebugWidget)),
        ("debug", Some(matches)) => {
            if matches.is_present("show") {
                Ok(Action::Ui(UiAction::ShowDebugWidget))
            } else if matches.is_present("hide") {
                Ok(Action::Ui(UiAction::HideDebugWidget))
            } else {
                Ok(Action::Ui(UiAction::ToggleDebugWidget))
            }
        }
        ("prompt", None) => Ok(Action::Ui(UiAction::TogglePrompt)),
        ("prompt", Some(matches)) => {
            if matches.is_present("show") {
                Ok(Action::Ui(UiAction::ShowPrompt))
            } else if matches.is_present("hide") {
                Ok(Action::Ui(UiAction::HidePrompt))
            } else {
                Ok(Action::Ui(UiAction::TogglePrompt))
            }
        }
        (arg, _) => Err(ParseActionError::UnknownArgument(arg.to_string())),
    }
}
