use serde_json::{json, Map, Value};

use std::io::Result as IoResult;
use std::path::{Path, PathBuf};

use super::ClientImpl;
use crate::xi::Message;
use crate::xi::ViewId;

/// Contains all methods used for sending and receiving to xi-core.
#[async_trait::async_trait]
pub trait ClientExt: ClientImpl {
    /// Send the given `value` to xi-core.
    async fn to(&mut self, value: Value) -> IoResult<()> {
        ClientImpl::send(self, value).await
    }

    /// Attempt to read a value from xi-core.
    async fn get(&mut self) -> IoResult<Message> {
        ClientImpl::receive(self).await
    }

    /// Send a notification to xi-core.
    async fn notify(&mut self, method: &str, params: Value) -> IoResult<()> {
        self.to(json!({"method": method, "params": params})).await?;
        Ok(())
    }

    /// Send a request to xi-core.
    async fn request(&mut self, method: &str, params: Value) -> IoResult<usize> {
        let req_id = self.next_id();
        self.to(json!({"id": req_id, "method": method, "params": params}))
            .await?;
        Ok(req_id)
    }

    /// Send a simple edit command to xi-core.
    /// example: ClientExt::simple_edit(ViewId(1), "move_right")
    /// { "method": "edit", "params": {"view_id": 1, "method":"move_right"}}
    async fn simple_edit(&mut self, view_id: ViewId, method: &str) -> IoResult<()> {
        self.notify("edit", json!({"view_id": view_id, "method": method}))
            .await
    }

    async fn edit(&mut self, view_id: ViewId, method: &str, params: Value) -> IoResult<()> {
        self.notify(
            "edit",
            json!({"view_id": view_id, "method": method, "params": params}),
        )
        .await
    }

    /// Sends the client_started RPC needed to begin using the xi editor.
    async fn client_started(
        &mut self,
        conf: Option<PathBuf>,
        extras: Option<PathBuf>,
    ) -> IoResult<()> {
        let mut map = Map::new();
        if let Some(path) = conf {
            map.insert("config_dir".into(), json!(path));
        }
        if let Some(path) = extras {
            map.insert("client_extras_dir".into(), json!(path));
        }
        self.notify("client_started", json!(map)).await
    }

    /// Sends the new_view RPC and returns the ViewId that has been created.
    async fn new_view(&mut self, file_path: Option<String>) -> IoResult<usize> {
        let mut map = Map::new();
        if let Some(file_path) = file_path {
            map.insert("file_path".into(), json!(file_path));
        }
        Ok(self.request("new_view", json!(map)).await?)
    }

    /// Sends the set_theme notification to xi-core.
    async fn set_theme(&mut self, theme: &str) -> IoResult<()> {
        self.notify("set_theme", json!({ "theme_name": theme }))
            .await
    }

    /// Sends the set_language notification to xi-core.
    async fn set_language(&mut self, id: ViewId, lang: &str) -> IoResult<()> {
        let mut map = Map::new();
        map.insert("language_id".into(), lang.into());
        map.insert("view_id".into(), json!(id));
        self.notify("set_theme", json!(map)).await
    }

    async fn scroll(&mut self, view: ViewId, x: u64, y: u64) -> IoResult<()> {
        self.edit(view, "scroll", json!([x, y])).await
    }

    async fn resize(&mut self, view: ViewId, x: u64, y: u64) -> IoResult<()> {
        self.edit(view, "resize", json!({"width":x, "height":y}))
            .await
    }

    async fn save(&mut self, view: ViewId, file_path: &Path) -> IoResult<()> {
        self.notify("save", json!({"file_path":file_path, "view_id": view}))
            .await
    }

    async fn find(
        &mut self,
        view: ViewId,
        query: &str,
        case: bool,
        regex: bool,
        words: bool,
    ) -> IoResult<()> {
        self.edit(
            view,
            "find",
            json!({
                "chars": query,
                "case_sensitive": case,
                "regex": regex,
                "whole_words": words
            }),
        )
        .await
    }

    async fn highlight_find(&mut self, view: ViewId, visible: bool) -> IoResult<()> {
        self.edit(view, "highlight_find", json!({ "visible": visible }))
            .await
    }

    /// Sends the insert notification to xi-core
    async fn insert(&mut self, id: ViewId, data: &str) -> IoResult<()> {
        let data = json!({
            "method": "insert",
            "view_id": id,
            "params": {
                "chars": data
            }
        });
        self.notify("edit", data).await
    }
}

impl<C: ClientImpl> ClientExt for C {}
