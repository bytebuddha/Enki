use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent};

use std::num::ParseIntError;

#[derive(Debug, PartialEq, Clone)]
pub enum ParseEventError {
    Int(ParseIntError),
    UnknownKey(String),
    DuplicateKey(String),
    ExpectedDelimeter(char),
    MissingCode,
}

impl From<ParseIntError> for ParseEventError {
    fn from(err: ParseIntError) -> ParseEventError {
        ParseEventError::Int(err)
    }
}

pub fn parse_event(raw_input: &str) -> Result<Event, ParseEventError> {
    let mut modifiers = KeyModifiers::empty();
    let mut key_code = None;
    let mut mouse = None;

    let input = raw_input.trim().to_lowercase();
    if &input == "resize" {
        return Ok(Event::Resize(0, 0));
    }

    let inputs: Vec<&str> = input.split(' ').collect();

    for input in inputs {
        match input {
            "control" => {
                if !modifiers.contains(KeyModifiers::CONTROL) {
                    modifiers.insert(KeyModifiers::CONTROL);
                } else {
                    return Err(ParseEventError::DuplicateKey(input.into()));
                }
            }
            "shift" => {
                if !modifiers.contains(KeyModifiers::SHIFT) {
                    modifiers.insert(KeyModifiers::SHIFT);
                } else {
                    return Err(ParseEventError::DuplicateKey(input.into()));
                }
            }
            "alt" => {
                if !modifiers.contains(KeyModifiers::ALT) {
                    modifiers.insert(KeyModifiers::ALT);
                } else {
                    return Err(ParseEventError::DuplicateKey(input.into()));
                }
            }
            "home" => {
                if key_code.is_some() {
                    return Err(ParseEventError::DuplicateKey("home".into()));
                } else {
                    key_code = Some(KeyCode::Home);
                }
            }
            "end" => {
                if key_code.is_some() {
                    return Err(ParseEventError::DuplicateKey("end".into()));
                } else {
                    key_code = Some(KeyCode::End);
                }
            }
            "delete" => {
                if key_code.is_some() {
                    return Err(ParseEventError::DuplicateKey("delete".into()));
                } else {
                    key_code = Some(KeyCode::Delete);
                }
            }
            "backspace" => {
                if key_code.is_some() {
                    return Err(ParseEventError::DuplicateKey("backspace".into()));
                } else {
                    key_code = Some(KeyCode::Backspace);
                }
            }
            "pageup" => {
                if key_code.is_some() {
                    return Err(ParseEventError::DuplicateKey("pageup".into()));
                } else {
                    key_code = Some(KeyCode::PageUp);
                }
            }
            "pagedown" => {
                if key_code.is_some() {
                    return Err(ParseEventError::DuplicateKey("pagedown".into()));
                } else {
                    key_code = Some(KeyCode::PageDown);
                }
            }
            "enter" => {
                if key_code.is_some() {
                    return Err(ParseEventError::DuplicateKey("enter".into()));
                } else {
                    key_code = Some(KeyCode::Enter);
                }
            }
            "escape" => {
                if key_code.is_some() {
                    return Err(ParseEventError::DuplicateKey("escape".into()));
                } else {
                    key_code = Some(KeyCode::Esc);
                }
            }
            "backtab" => {
                if key_code.is_some() {
                    return Err(ParseEventError::DuplicateKey("backtab".into()));
                } else {
                    key_code = Some(KeyCode::BackTab);
                }
            }
            "insert" => {
                if key_code.is_some() {
                    return Err(ParseEventError::DuplicateKey("insert".into()));
                } else {
                    key_code = Some(KeyCode::Insert);
                }
            }
            "tab" => {
                if key_code.is_some() {
                    return Err(ParseEventError::DuplicateKey("tab".into()));
                } else {
                    key_code = Some(KeyCode::Tab);
                }
            }
            "null" => {
                if key_code.is_some() {
                    return Err(ParseEventError::DuplicateKey("null".into()));
                } else {
                    key_code = Some(KeyCode::Null);
                }
            }
            "scroll_down" => {
                mouse = Some(MouseEvent::ScrollDown(0, 0, KeyModifiers::empty()));
            }
            "scroll_up" => {
                mouse = Some(MouseEvent::ScrollUp(0, 0, KeyModifiers::empty()));
            }
            raw_input => {
                if raw_input == "down(leftbtn)" {
                    mouse = Some(MouseEvent::Down(
                        MouseButton::Left,
                        0,
                        0,
                        KeyModifiers::empty(),
                    ));
                    continue;
                }
                if raw_input == "down(rightbtn)" {
                    mouse = Some(MouseEvent::Down(
                        MouseButton::Right,
                        0,
                        0,
                        KeyModifiers::empty(),
                    ));
                    continue;
                }
                if raw_input == "down(middlebtn)" {
                    mouse = Some(MouseEvent::Down(
                        MouseButton::Middle,
                        0,
                        0,
                        KeyModifiers::empty(),
                    ));
                    continue;
                }
                if raw_input == "up(leftbtn)" {
                    mouse = Some(MouseEvent::Up(
                        MouseButton::Left,
                        0,
                        0,
                        KeyModifiers::empty(),
                    ));
                    continue;
                }
                if raw_input == "up(rightbtn)" {
                    mouse = Some(MouseEvent::Up(
                        MouseButton::Right,
                        0,
                        0,
                        KeyModifiers::empty(),
                    ));
                    continue;
                }
                if raw_input == "up(middlebtn)" {
                    mouse = Some(MouseEvent::Up(
                        MouseButton::Middle,
                        0,
                        0,
                        KeyModifiers::empty(),
                    ));
                    continue;
                }
                let input = raw_input.trim();
                if &input[0..1] == "f" {
                    if input.len() == 1 {
                        key_code = Some(KeyCode::Char('f'));
                    } else if let Ok(num) = input[1..].parse() {
                        key_code = Some(KeyCode::F(num));
                    } else {
                        return Err(ParseEventError::UnknownKey(input.into()));
                    }
                } else if input.len() == 1 {
                    key_code = Some(KeyCode::Char(input.chars().next().unwrap()));
                } else {
                    return Err(ParseEventError::UnknownKey(input.into()));
                }
            }
        }
    }

    if let Some(code) = key_code {
        Ok(Event::Key(KeyEvent { modifiers, code }))
    } else if let Some(mouse) = mouse {
        match mouse {
            MouseEvent::Up(btn, _, _, _) => Ok(Event::Mouse(MouseEvent::Up(btn, 0, 0, modifiers))),
            MouseEvent::Down(btn, _, _, _) => {
                Ok(Event::Mouse(MouseEvent::Down(btn, 0, 0, modifiers)))
            }
            MouseEvent::Drag(btn, _, _, _) => {
                Ok(Event::Mouse(MouseEvent::Drag(btn, 0, 0, modifiers)))
            }
            MouseEvent::ScrollUp(_, _, _) => {
                Ok(Event::Mouse(MouseEvent::ScrollUp(0, 0, modifiers)))
            }
            MouseEvent::ScrollDown(_, _, _) => {
                Ok(Event::Mouse(MouseEvent::ScrollDown(0, 0, modifiers)))
            }
        }
    } else {
        Err(ParseEventError::MissingCode)
    }
}
