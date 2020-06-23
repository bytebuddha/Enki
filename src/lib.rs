#![feature(associated_type_defaults)]
extern crate crossterm;
extern crate serde;
extern crate serde_json;
extern crate tokio;

pub mod actions;
pub mod core;
pub mod debug;
pub mod editor;
pub mod enki;
pub mod prompt;
pub mod textarea;
pub mod view;
pub mod xi;

use crate::core::{EnkiEvent, LoopResponse, Result};
use crate::enki::StartUpFile;
use actions::{Action, EditorAction};
use log::trace;
use std::path::PathBuf;

pub async fn run() -> Result<()> {
    // Get command line arguments
    let arg_matches = core::enki_cli_args();

    // Get the configuration directory.
    let conf_dir = arg_matches
        .value_of("conf")
        .map(PathBuf::from)
        .or(core::get_config_directory()?);

    let files = arg_matches
        .values_of("files")
        .unwrap()
        .map(ToString::to_string)
        .collect::<Vec<String>>();

    let (level, startup_file) = StartUpFile::new(conf_dir.as_ref()).apply_arguments(&arg_matches);
    if let Some(log_file) = arg_matches.value_of("log_file") {
        core::configure_logs(level, log_file);
    }

    let mut enki = enki::Enki::new(conf_dir, startup_file).await?;

    for file in files {
        enki.handle_action(Action::Editor(EditorAction::Open(Some(file))))
            .await?;
    }

    main_loop(enki).await
}

pub async fn main_loop(mut enki: enki::Enki<'_>) -> Result<()> {
    'main: loop {
        if let Some(event) = enki.next_event().await? {
            trace!("Handling Event: {:?}", event);
            let response = match event {
                EnkiEvent::XiMessage(msg) => enki.handle_rpc(msg).await?,
                EnkiEvent::Action(action) => enki.handle_action(action).await?,
                EnkiEvent::Actions(actions) => {
                    let mut needs_render = false;
                    for action in actions {
                        match enki.handle_action(action).await? {
                            LoopResponse::Quit => break 'main,
                            LoopResponse::Render => needs_render = true,
                            _ => {}
                        }
                    }
                    if needs_render {
                        LoopResponse::Render
                    } else {
                        LoopResponse::Continue
                    }
                }
                EnkiEvent::Terminal(crossterm::event::Event::Resize(x, y)) => {
                    enki.interface.handle_resize(&mut enki.client, x, y).await?;
                    LoopResponse::Continue
                }
                EnkiEvent::Terminal(event) => {
                    let mut needs_render = false;
                    if let Some(actions) = enki.reactor.react(&event) {
                        for action in actions {
                            match enki.handle_action(action).await? {
                                LoopResponse::Quit => break 'main,
                                LoopResponse::Render => needs_render = true,
                                _ => {}
                            }
                        }
                    } else if let Some(action) = enki
                        .interface
                        .handle_event(&mut enki.arguments, event)
                        .await?
                    {
                        match enki.handle_action(action).await? {
                            LoopResponse::Quit => break 'main,
                            LoopResponse::Render => needs_render = true,
                            _ => {}
                        }
                    }
                    if needs_render {
                        LoopResponse::Render
                    } else {
                        LoopResponse::Continue
                    }
                }
            };
            trace!("Response: {:?}", response);
            match response {
                LoopResponse::Quit => break,
                LoopResponse::Render => enki.render()?,
                LoopResponse::Continue => {}
            }
        }
    }
    Ok(())
}
