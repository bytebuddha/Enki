mod logs;
pub use self::logs::{configure_logs, LogLevel};

mod error;
pub use self::error::Error;

pub mod consts;
pub mod utils;

use crate::xi::Message;
use clap::{clap_app, crate_description, crate_version, ArgMatches};
use crossterm::event::Event;
use xdg::BaseDirectories;

use crate::actions::Action;
use crate::enki::Context;

use std::path::PathBuf;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum EnkiEvent {
    Actions(Vec<Action>),
    Action(Action),
    XiMessage(Message),
    Terminal(Event),
}

#[derive(Debug)]
pub enum LoopResponse {
    Quit,
    Render,
    Continue,
}

pub fn get_config_directory() -> crate::Result<Option<PathBuf>> {
    let directory = BaseDirectories::with_prefix("enki").map(|item| item.get_config_home())?;
    if directory.exists() {
        Ok(Some(directory))
    } else {
        Ok(None)
    }
}

pub fn enki_cli_args<'a>() -> ArgMatches<'a> {
    clap_app!(enki =>
        (version: crate_version!())
        (about: crate_description!())
        (@arg verbose: -v --verbose +multiple requires[log_file] "Set the verbosity of the log file")
        (@arg log_file: -l --log +takes_value "Debug Log File to write")
        (@arg xi: -x --xi +takes_value "path to the core to use")
        (@arg conf: -c --conf +takes_value "Path to the configuration directory")
        (@arg files: +required +multiple "File names to open")
    ).get_matches()
}

#[async_trait::async_trait]
pub trait EventHandler {
    type Type = Option<Action>;
    async fn handle_event(&mut self, event: Event) -> crate::Result<Self::Type>;
}

#[async_trait::async_trait]
pub trait ActionHandler {
    type Event = Action;

    async fn handle_action<'a, 'b, 'c>(
        &mut self,
        context: Context<'a, 'b, 'c>,
        event: Self::Event,
    ) -> crate::Result<LoopResponse>;
}
