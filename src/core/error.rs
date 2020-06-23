use crate::xi::ViewId;
use crossterm::ErrorKind as TerminalError;
use serde_json::Error as JsonError;
use xdg::BaseDirectoriesError;

use crate::actions::ParseActionError;
use crate::actions::ParseEventError;

use std::io::Error as IoError;

#[derive(Debug)]
pub enum Error {
    UnknownView(ViewId),
    Io(IoError),
    Json(JsonError),
    XiCore(String),
    Terminal(TerminalError),
    Xdg(BaseDirectoriesError),
    EventParse(ParseEventError),
    ActionParse(ParseActionError),
}

macro_rules! impl_simple_from {
    ($err: ident, $value: ident) => {
        impl From<$err> for Error {
            fn from(err: $err) -> Error {
                Error::$value(err)
            }
        }
    };
}

impl_simple_from!(BaseDirectoriesError, Xdg);
impl_simple_from!(JsonError, Json);
impl_simple_from!(IoError, Io);
impl_simple_from!(TerminalError, Terminal);
impl_simple_from!(ParseActionError, ActionParse);
