#[derive(Debug, PartialEq, Clone)]
pub enum EditorAction {
    ListViews,
    ListThemes,
    ListPlugins,
    ListLanguages,
    NextView,
    PreviousView,
    Open(Option<String>),
    SetTheme(String),
}
