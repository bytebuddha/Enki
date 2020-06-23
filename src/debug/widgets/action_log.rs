use derive_builder::Builder;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::{List, Text, Widget};

use crate::debug::Actions;

#[derive(Builder)]
pub struct ActionLogWidget<'a> {
    actions: &'a Actions,
}

impl<'a> Widget for ActionLogWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vars = self.actions.actions();
        let iter = vars.iter().map(|item| Text::raw(format!("{:?}", item)));
        List::new(iter).render(area, buf);
    }
}
