use crate::textarea::TextArea;
use crate::xi::ViewId;
use tui::layout::Rect;

mod actions;
mod event;
mod widgets;
pub use self::widgets::{ViewWidget, ViewWidgetBuilder};

pub struct View {
    pub language: Option<String>,
    pub textarea: TextArea,
    pub plugins: Vec<String>,
    pub area: Option<Rect>,
}

impl View {
    pub fn new(id: ViewId) -> View {
        View {
            language: None,
            textarea: TextArea::new(id),
            plugins: vec![],
            area: None,
        }
    }

    pub fn calculate_gutter_rect(&self, rect: Rect) -> Rect {
        Rect {
            x: rect.x,
            y: rect.y,
            width: 5,
            height: rect.height,
        }
    }

    pub fn calculate_view_rect(&self, rect: Rect) -> Rect {
        Rect {
            x: rect.x + 5,
            y: rect.y,
            width: rect.width - 5,
            height: rect.height,
        }
    }

    pub fn calculate_status_rect(&self, rect: Rect) -> Rect {
        let y = rect.y + rect.height - 1;
        Rect {
            x: rect.x,
            y,
            height: 1,
            width: rect.width,
        }
    }
}
