use crate::core::consts::DISPLAY_TOP_BAR;
use crate::enki::Configuration;
use derive_builder::Builder;
use log::debug;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::{StatefulWidget, Widget};

mod top_bar;
pub use self::top_bar::TopBarBuilder;

use crate::editor::Editor;
use crate::view::ViewWidgetBuilder;

#[derive(Builder)]
pub struct EditorWidget<'a> {
    pub config: &'a Configuration,
}

impl<'a> StatefulWidget for EditorWidget<'a> {
    type State = Editor;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let view_rect = if self
            .config
            .get_default_t("display_top_bar", DISPLAY_TOP_BAR)
        {
            let top_bar_rect = state.calculate_topbar_rect(area);

            TopBarBuilder::default()
                .editor(&state)
                .build()
                .expect("Failed to build top bar widget")
                .render(top_bar_rect, buf);

            state.calculate_view_rect(area)
        } else {
            area
        };
        if let Some(mut view) = state.views.get_current_mut() {
            ViewWidgetBuilder::default()
                .theme(state.theme.as_ref().map(|item| &item.theme))
                .styles(&state.styles)
                .global_config(&self.config)
                .build()
                .expect("Failed to build View Widget")
                .render(view_rect, buf, &mut view);
            debug!("Seting Editor View State Rect: {:?}", view.area);
            state.view_rect = view.area.unwrap();
        }
    }
}
