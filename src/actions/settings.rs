use crossterm::event::Event;
use serde_json::Value;

use super::Action;

#[derive(Debug, PartialEq, Clone)]
pub enum SettingsAction {
    ConfigGet(String),
    ConfigBind(String, Value),
    ConfigUnbind(String),
    EventBind(Event, Vec<Action>),
    EventUnbind(Event),
}
