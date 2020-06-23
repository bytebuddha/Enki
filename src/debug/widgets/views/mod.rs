use tui::buffer::Buffer;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::widgets::{StatefulWidget, Widget};

use crate::debug::Debug;
use crate::editor::Editor;

mod view;
pub use self::view::ViewDebugWidgetBuilder;

mod view_list;
pub use self::view_list::ViewListWidgetBuilder;

mod plugins;
pub use self::plugins::PluginsWidgetBuilder;

mod text_area;
pub use self::text_area::TextAreaDebugWidget;

mod config;
pub use self::config::ConfigWidgetBuilder;

mod view_details;
pub use self::view_details::ViewDetailsWidgetBuilder;

pub struct ViewsDebugWidget<'a, 'b> {
    pub editor: &'a Editor,
    pub debug: &'b mut Debug,
}

impl<'a, 'b> ViewsDebugWidget<'a, 'b> {
    pub fn new(editor: &'a Editor, debug: &'b mut Debug) -> ViewsDebugWidget<'a, 'b> {
        ViewsDebugWidget { editor, debug }
    }
}

impl<'a, 'b> Widget for ViewsDebugWidget<'a, 'b> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
            .split(area);
        ViewListWidgetBuilder::default()
            .views(&self.editor.views)
            .build()
            .expect("Failed to build View List Widget")
            .render(chunks[0], buf, &mut self.debug.view_list_state);

        if let Some(selected) = self.debug.view_list_state.selected() {
            if let Some(view) = self.editor.views.get_index(selected) {
                ViewDebugWidgetBuilder::default()
                    .view(view)
                    .build()
                    .expect("Failed to build View Debug Widget")
                    .render(chunks[1], buf);
            }
        }
    }
}
