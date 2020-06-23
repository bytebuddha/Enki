#[derive(Debug, PartialEq, Clone)]
pub enum CursorAction {
    Up,
    Down,
    Left,
    LeftWord,
    Right,
    RightWord,
    PageUp,
    PageDown,
    Home,
    End,
    Backspace,
    Delete,
}
