use std::path::PathBuf;

use super::CursorAction;

#[derive(Debug, PartialEq, Clone)]
pub enum ViewAction {
    Save(Option<PathBuf>),
    SetLanguage(String),
    Cursor(CursorAction),
    Find(FindAction),
    Insert(String),
    Undo,
    Redo,
    Enter,
    Tab,
    BackTab,
}

#[derive(Debug, PartialEq, Clone)]
pub enum FindAction {
    Query(String, bool, bool, bool),
    Next(bool, bool),
    Previous(bool, bool),
}
