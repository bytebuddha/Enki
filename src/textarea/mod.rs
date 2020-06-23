use crate::core::LoopResponse;
use crate::enki::Terminal;
use crate::xi::{ConfigChanges, LineCache, ScrollTo, ViewId};
use log::debug;
use tui::layout::Rect;

mod actions;
mod widgets;
pub use self::widgets::{
    ChunkWidget, ChunkWidgetBuilder, LineWidget, LineWidgetBuilder, TextAreaWidget,
    TextAreaWidgetBuilder,
};

pub struct TextArea {
    pub id: ViewId,
    pub area: Option<Rect>,
    pub cache: LineCache,
    pub offset: usize,
    pub cursor: (u16, u16),
    pub config: Option<ConfigChanges>,
}

impl TextArea {
    pub fn new(id: ViewId) -> TextArea {
        let cache = LineCache::new();
        let config = None;
        let area = None;
        let offset = 0;
        let cursor = (1, 1);
        TextArea {
            id,
            area,
            cache,
            offset,
            cursor,
            config,
        }
    }

    pub async fn scroll_to(
        &mut self,
        scroll: ScrollTo,
        term: &mut Terminal,
    ) -> crate::core::Result<LoopResponse> {
        if let Some(area) = self.area {
            let line = scroll.line;
            let offset = self.offset as u64;
            let height = area.height as u64;
            let mut response = LoopResponse::Continue;
            if line >= offset && line - offset >= height {
                debug!("Scroll is below the view window");
                self.offset = line as usize - height as usize + 1;
                response = LoopResponse::Render;
            } else if line < offset && offset > 0 {
                debug!("Scroll is above the view window");
                self.offset -= 1;
                response = LoopResponse::Render;
            }
            self.cursor = (
                scroll.column as u16 + area.x,
                line as u16 - self.offset as u16 + area.y,
            );
            debug!("New View Offset: {:?}", self.offset);
            debug!("Setting Cursor to: {:?}", self.cursor);
            term.backend.set_cursor(self.cursor.0, self.cursor.1)?;
            Ok(response)
        } else {
            Ok(LoopResponse::Continue)
        }
    }
}
