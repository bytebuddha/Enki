use log::error;
use tui::layout::Rect;

use crate::actions::ViewAction;
use crate::core::{ActionHandler, LoopResponse};
use crate::enki::Context;
use crate::view::View;
use crate::xi::{Client, ClientExt, Plugin, StyleCache, ThemeChanged, ViewId, XiNotification};

mod actions;
mod event;
mod widgets;
pub use self::widgets::EditorWidgetBuilder;
mod view_list;
pub use self::view_list::ViewList;

#[derive(Default)]
pub struct Editor {
    pub styles: StyleCache,
    pub views: ViewList,
    pub languages: Vec<String>,
    pub theme: Option<ThemeChanged>,
    pub themes: Vec<String>,
    pub plugins: Vec<Plugin>,
    pub view_rect: Rect,
}

impl Editor {
    pub fn calculate_view_rect(&self, rect: Rect) -> Rect {
        Rect {
            x: rect.x,
            y: rect.y + 1,
            height: rect.height - 1,
            width: rect.width,
        }
    }

    pub fn calculate_topbar_rect(&self, rect: Rect) -> Rect {
        Rect {
            x: rect.x,
            y: rect.y,
            height: 1,
            width: rect.width,
        }
    }

    pub fn with<F: FnOnce(&mut View)>(&mut self, id: ViewId, func: F) {
        if let Some(item) = self.views.get_mut(&id) {
            func(item)
        }
    }

    pub fn with_current<F: FnOnce(&mut View)>(&mut self, func: F) {
        if let Some(view) = self.views.get_current_mut() {
            func(view);
        } else {
            error!("Current View was not set");
        }
    }

    pub fn get_views_list_string(&self) -> String {
        format!(
            "{:?}",
            self.views.keys().collect::<Vec<&crate::xi::ViewId>>()
        )
    }

    pub fn get_plugins_list_string(&self) -> String {
        format!(
            "{:?}",
            self.plugins
                .iter()
                .map(|item| {
                    if item.running {
                        format!("{}:active", item.name)
                    } else {
                        format!("{}:inactive", item.name)
                    }
                })
                .collect::<Vec<String>>()
        )
    }

    pub async fn handle_resize(
        &mut self,
        client: &mut Client,
        x: u64,
        y: u64,
    ) -> crate::core::Result<()> {
        for view in self.views.get_all() {
            if self.view_rect.height == 0 {
                client.scroll(view.textarea.id, x, y).await?;
            } else {
                client
                    .scroll(view.textarea.id, x, self.view_rect.height as u64)
                    .await?;
            }
        }
        Ok(())
    }

    pub fn handle_rpc(&mut self, rpc: XiNotification) -> crate::core::Result<LoopResponse> {
        match rpc {
            XiNotification::DefStyle(style) => {
                self.styles.insert(style.id, style);
            }
            XiNotification::ConfigChanged(changes) => self.with(changes.view_id, |view| {
                view.textarea.config = Some(changes.changes)
            }),
            XiNotification::AvailableLanguages(languages) => self.languages = languages.languages,
            XiNotification::AvailablePlugins(plugins) => self.plugins = plugins.plugins,
            XiNotification::AvailableThemes(themes) => self.themes = themes.themes,
            XiNotification::ThemeChanged(theme) => self.theme = Some(theme),
            XiNotification::LanguageChanged(lang) => {
                self.with(lang.view_id, |view| view.language = Some(lang.language_id))
            }
            XiNotification::PluginStarted(started) => {
                self.with(started.view_id, |view| view.plugins.push(started.plugin))
            }
            XiNotification::PluginStoped(stoped) => self.with(stoped.view_id, |view| {
                view.plugins.retain(|item| item != &stoped.plugin)
            }),
            XiNotification::Update(update) => {
                self.with(update.view_id, |view| {
                    view.textarea.cache.update(update.update)
                });
                return Ok(LoopResponse::Render);
            }
            _ => {}
        }
        Ok(LoopResponse::Continue)
    }

    pub async fn handle_view_action<'a, 'b, 'c>(
        &mut self,
        context: Context<'a, 'b, 'c>,
        view: Option<ViewId>,
        action: ViewAction,
    ) -> crate::core::Result<LoopResponse> {
        let view = if let Some(view) = view {
            self.views.get_mut(&view)
        } else {
            self.views.get_current_mut()
        };
        if let Some(view) = view {
            view.handle_action(context, action).await
        } else {
            error!("No Current View");
            Ok(LoopResponse::Continue)
        }
    }

    pub fn new_view(&mut self, view_id: ViewId) -> crate::core::Result<LoopResponse> {
        self.views.add(View::new(view_id));
        Ok(LoopResponse::Continue)
    }
}
