mod config;
pub use self::config::Configuration;

mod context;
pub use self::context::Context;

mod reactor;
pub use self::reactor::ActionReactor;

mod startup;
pub use self::startup::StartUpFile;

mod terminal;
pub use self::terminal::Terminal;

mod interface;
pub use self::interface::UserInterface;

use clap::App;
use log::{debug, info};
use serde_json::{json, Value};

use crate::xi::client::{Client, ClientExt};
use crate::xi::{Message, XiLocation, XiNotification};

use crate::actions::{get_arguments, parse_action, Action, EditorAction, SettingsAction, UiAction};
use crate::prompt::Message as PromptMessage;

use crate::core::{utils::run_shell_command, ActionHandler, EnkiEvent, LoopResponse};

use std::collections::HashMap;
use std::path::PathBuf;

pub enum ActiveRequest {
    NewView(Value),
}

pub struct Enki<'a> {
    pub arguments: App<'a, 'a>,
    pub client: Client,
    pub requests: HashMap<usize, ActiveRequest>,
    pub interface: UserInterface,
    pub terminal: Terminal,
    pub config: Configuration,
    pub reactor: ActionReactor,
}

impl<'a> Enki<'a> {
    pub async fn new(
        conf_dir: Option<PathBuf>,
        start: StartUpFile,
    ) -> crate::core::Result<Enki<'a>> {
        let arguments = get_arguments();
        let core_path = start.core.unwrap_or(XiLocation::Embeded);

        info!("Launching Xi Core: {:?}", core_path);

        let config = start.config.map(Configuration::from).unwrap_or_default();

        let reactor = start
            .default_actions
            .filter(|item| !*item)
            .map(|_| ActionReactor::new())
            .unwrap_or_default();

        let mut enki = Enki {
            arguments,
            config,
            reactor,
            client: Client::new(core_path)?,
            interface: UserInterface::default(),
            terminal: Terminal::new()?,
            requests: HashMap::new(),
        };

        let extras_dir = if let Some(dir) = &conf_dir {
            let mut temp_dir = dir.clone();
            temp_dir.push("extras");
            Some(temp_dir)
        } else {
            None
        };

        let config_dir = conf_dir.map(|mut item| {
            item.push("xi");
            item
        });
        info!("Using Config Dir: {:?}", config_dir);
        info!("Using Extras Dir: {:?}", extras_dir);
        enki.client.client_started(config_dir, extras_dir).await?;
        enki.client.set_theme("base16-eighties.dark").await?;

        if let Some(actions) = start.actions {
            for action in actions {
                let action = parse_action(&mut enki.arguments, &action)?;
                enki.handle_action(action).await?;
            }
        }
        Ok(enki)
    }

    pub async fn next_event(&mut self) -> crate::core::Result<Option<EnkiEvent>> {
        tokio::select! {
            event = self.client.get() => {
                Ok(Some(EnkiEvent::XiMessage(event?)))
            },
            event = self.terminal.next_event() => {
                if let Some(event) = event {
                    if let Some(actions) = self.reactor.react(&event) {
                        Ok(Some(EnkiEvent::Actions(actions)))
                    } else {
                        Ok(Some(EnkiEvent::Terminal(event)))
                    }
                } else {
                    Ok(None)
                }
            }
        }
    }

    pub async fn handle_action(&mut self, action: Action) -> crate::core::Result<LoopResponse> {
        self.interface.debug.actions.push(action.clone());
        let context = Context {
            client: &mut self.client,
            config: &mut self.config,
            reactor: &mut self.reactor,
        };
        match action {
            Action::Quit => Ok(LoopResponse::Quit),
            Action::ShellCommand(cmd) => {
                let data = run_shell_command(cmd)?;
                self.interface
                    .set_prompt_message(PromptMessage::info(data).title(" Shell Output "));
                Ok(LoopResponse::Render)
            }
            Action::Ui(UiAction::Render) => Ok(LoopResponse::Render),
            Action::Ui(action) => self.interface.handle_action(context, action),
            Action::Settings(action) => Enki::handle_settings_action(context, action).await,
            Action::View(view, action) => {
                self.interface
                    .editor
                    .handle_view_action(context, view, action)
                    .await
            }
            Action::Editor(EditorAction::ListViews) => {
                let data = self.interface.editor.get_views_list_string();
                self.interface
                    .set_prompt_message(PromptMessage::info(data).title(" Views "));
                Ok(LoopResponse::Render)
            }
            Action::Editor(EditorAction::ListLanguages) => {
                let data = format!("{:?}", self.interface.editor.languages);
                self.interface
                    .set_prompt_message(PromptMessage::info(data).title(" Languages "));
                Ok(LoopResponse::Render)
            }
            Action::Editor(EditorAction::ListThemes) => {
                let data = format!("{:?}", self.interface.editor.themes);
                self.interface
                    .set_prompt_message(PromptMessage::info(data).title(" Themes "));
                Ok(LoopResponse::Render)
            }
            Action::Editor(EditorAction::ListPlugins) => {
                let data = self.interface.editor.get_plugins_list_string();
                self.interface
                    .set_prompt_message(PromptMessage::info(data).title(" Plugins "));
                Ok(LoopResponse::Render)
            }
            Action::Editor(EditorAction::Open(file)) => {
                let id = context.client.new_view(file.to_owned()).await?;
                self.requests.insert(id, ActiveRequest::NewView(json!(id)));
                Ok(LoopResponse::Continue)
            }
            Action::Editor(action) => self.interface.editor.handle_action(context, action).await,
        }
    }

    pub async fn handle_rpc(&mut self, rpc: Message) -> crate::core::Result<LoopResponse> {
        match rpc {
            Message::Notification(XiNotification::ScrollTo(scroll)) => {
                Ok(self.interface.scroll_to(scroll, &mut self.terminal).await?)
            }
            Message::Notification(notification) => self.interface.editor.handle_rpc(notification),
            Message::Response(response) => {
                if let Some(request) = self.requests.get(&(response.id as usize)) {
                    match response.result {
                        Err(_err) => Ok(LoopResponse::Continue),
                        Ok(response) => match request {
                            ActiveRequest::NewView(_request) => {
                                let view_id = serde_json::from_value(response)?;
                                debug!("Opening New View: {:?}", view_id);
                                let view_rect = self.interface.editor.view_rect;
                                if view_rect.area() == 0 {
                                    let size = self.terminal.size()?;
                                    self.client.scroll(view_id, 0, size.1 as u64 - 1).await?;
                                } else {
                                    self.client
                                        .scroll(view_id, 0, view_rect.height as u64 - 1)
                                        .await?;
                                }
                                self.interface.editor.new_view(view_id)
                            }
                        },
                    }
                } else {
                    Ok(LoopResponse::Continue)
                }
            }
            Message::Error(_) => Ok(LoopResponse::Continue),
            Message::Request(_) => unreachable!(),
        }
    }

    pub async fn handle_settings_action<'b, 'c>(
        context: Context<'a, 'b, 'c>,
        action: SettingsAction,
    ) -> crate::core::Result<LoopResponse> {
        match action {
            SettingsAction::ConfigBind(key, value) => context.config.set(key, value),
            SettingsAction::ConfigUnbind(key) => context.config.unset(key),
            SettingsAction::EventBind(event, action) => context.reactor.bind(event, action),
            SettingsAction::EventUnbind(event) => context.reactor.unbind(event),
            SettingsAction::ConfigGet(_) => {}
        }
        Ok(LoopResponse::Continue)
    }

    pub fn render(&mut self) -> crate::core::Result<()> {
        let context = Context {
            client: &mut self.client,
            config: &mut self.config,
            reactor: &mut self.reactor,
        };
        self.interface.render(context, &mut self.terminal)?;
        Ok(())
    }
}
