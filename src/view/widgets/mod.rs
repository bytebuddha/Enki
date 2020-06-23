use crate::core::consts::{DISPLAY_GUTTER, DISPLAY_STATUS_BAR};
use crate::enki::Configuration;
use crate::xi::{StyleCache, ThemeSettings};
use derive_builder::Builder;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::{StatefulWidget, Widget};

use crate::textarea::TextAreaWidgetBuilder;
use crate::view::View;

mod gutter;
pub use self::gutter::GutterBuilder;

mod status_bar;
pub use self::status_bar::StatusBarBuilder;

#[derive(Builder)]
pub struct ViewWidget<'a, 'b, 'c> {
    pub global_config: &'a Configuration,
    #[builder(default)]
    pub theme: Option<&'b ThemeSettings>,
    #[builder(default)]
    #[builder(setter(strip_option))]
    pub styles: Option<&'c StyleCache>,
}

impl<'a, 'b, 'c> StatefulWidget for ViewWidget<'a, 'b, 'c> {
    type State = View;

    fn render(self, rect: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let display_status_bar = self
            .global_config
            .get_default_t("display_status_bar", DISPLAY_STATUS_BAR);
        if self
            .global_config
            .get_default_t("display_gutter", DISPLAY_GUTTER)
        {
            let mut gutter_rect = state.calculate_gutter_rect(rect);
            if display_status_bar {
                gutter_rect = Rect {
                    x: gutter_rect.x,
                    y: gutter_rect.y,
                    width: gutter_rect.width,
                    height: gutter_rect.height - 1,
                };
            }
            GutterBuilder::default()
                .view(&state)
                .start(Some(state.textarea.offset as u64))
                .theme(self.theme)
                .build()
                .expect("Failed to build gutter widget")
                .render(gutter_rect, buf);

            let text_area_rect = state.calculate_view_rect(rect);
            if display_status_bar {
                let status_bar_rect = state.calculate_status_rect(rect);
                let text_area_rect = Rect {
                    x: text_area_rect.x,
                    y: text_area_rect.y,
                    height: text_area_rect.height - 1,
                    width: text_area_rect.width,
                };
                StatusBarBuilder::default()
                    .build()
                    .expect("Failed to build status bar widget")
                    .render(status_bar_rect, buf);
                TextAreaWidgetBuilder::default()
                    .theme(self.theme)
                    .styles(self.styles)
                    .global_config(&self.global_config)
                    .build()
                    .expect("Failed to build Text Area Widget")
                    .render(text_area_rect, buf, &mut state.textarea);
                state.area = Some(text_area_rect);
            } else {
                TextAreaWidgetBuilder::default()
                    .theme(self.theme)
                    .styles(self.styles)
                    .global_config(&self.global_config)
                    .build()
                    .expect("Failed to build Text Area Widget")
                    .render(text_area_rect, buf, &mut state.textarea);
                state.area = Some(text_area_rect);
            }
        } else {
            TextAreaWidgetBuilder::default()
                .theme(self.theme)
                .styles(self.styles)
                .global_config(&self.global_config)
                .build()
                .expect("Failed to build Text Area Widget")
                .render(rect, buf, &mut state.textarea);
            state.area = Some(rect);
        }
    }
}
