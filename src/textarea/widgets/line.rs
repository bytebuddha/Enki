use crate::core::consts::DISPLAY_LINE_ENDINGS;
use crate::core::utils::u32_to_color;
use crate::enki::Configuration;
use crate::xi::{ConfigChanges, Line, StyleCache, ThemeSettings};
use derive_builder::Builder;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::Widget;

use super::ChunkWidgetBuilder;

#[derive(Builder)]
pub struct LineWidget<'a, 'b, 'c, 'd, 'e> {
    #[builder(default)]
    styles: Option<&'b StyleCache>,
    #[builder(default)]
    theme: Option<&'c ThemeSettings>,
    #[builder(default)]
    line: Option<&'a Line>,
    #[allow(dead_code)]
    global_config: &'d Configuration,
    #[allow(dead_code)]
    config: Option<&'e ConfigChanges>,
}

impl<'a, 'b, 'c, 'd, 'e> LineWidget<'a, 'b, 'c, 'd, 'e> {
    fn get_line_text<'z>(&self, input: &'z str) -> &'z str {
        if self
            .global_config
            .get_default_t("display_line_endings", DISPLAY_LINE_ENDINGS)
        {
            input
        } else {
            if let Some(conf) = self.config {
                if let Some(line_ending) = &conf.line_ending {
                    if input.len() > line_ending.len() {
                        return &input[..input.len() - line_ending.len()];
                    }
                }
            }
            input.trim_end()
        }
    }
}

impl<'a, 'b, 'c, 'd, 'e> Widget for LineWidget<'a, 'b, 'c, 'd, 'e> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if let Some(line) = self.line {
            if line.styles.is_empty() {
                ChunkWidgetBuilder::default()
                    .text(self.get_line_text(&line.text))
                    .theme(self.theme)
                    .build()
                    .expect("Failed to create Chunk Widget")
                    .render(area, buf);
            } else {
                let line_text = self.get_line_text(&line.text);
                let mut current_step = area.x as i64;
                for style_def in &line.styles {
                    if let Some(styles) = self.styles {
                        if let Some(style) = styles.get(style_def.style_id) {
                            let start = style_def.offset + current_step;
                            let chunk_rect = Rect {
                                x: start as u16,
                                y: area.y,
                                width: style_def.length as u16,
                                height: area.height,
                            };

                            let line_start = (start - area.x as i64) as usize;

                            let tab_size = self
                                .config
                                .and_then(|item| item.tab_size)
                                .or_else(|| self.global_config.get_t("tab_size"))
                                .map(|item| item as u16);

                            if line_start < line_text.len() {
                                ChunkWidgetBuilder::default()
                                    .text(&line_text[line_start..])
                                    .background(style.bg_color.map(u32_to_color))
                                    .foreground(style.fg_color.map(u32_to_color))
                                    .theme(self.theme)
                                    .tab_size(tab_size)
                                    .italic(style.italic)
                                    .underlined(style.underline)
                                    .build()
                                    .expect("Failed to create chunk widget")
                                    .render(chunk_rect, buf);
                            }

                            current_step = start + style_def.length as i64;
                        }
                    }
                }
            }
        }
    }
}
