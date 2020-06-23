use derive_builder::Builder;
use tui::buffer::Buffer;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, StatefulWidget, Tabs, Widget};

use crate::debug::Debug;
use crate::editor::Editor;
use crate::enki::{ActionReactor, Configuration};

mod actions;
pub use self::actions::ActionsWidgetBuilder;

mod config;
pub use self::config::ConfigurationWidgetBuilder;

mod action_log;
pub use self::action_log::ActionLogWidgetBuilder;

mod views;
pub use self::views::ViewsDebugWidget;

#[derive(Builder)]
pub struct DebugWidget<'a, 'b, 'c> {
    pub editor: &'a Editor,
    pub reactor: &'b ActionReactor,
    pub config: &'c Configuration,
}

impl<'a, 'b, 'c> StatefulWidget for DebugWidget<'a, 'b, 'c> {
    type State = Debug;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let tab_bar_rect = Rect {
            x: 0,
            y: 0,
            height: 3,
            width: area.width,
        };
        let content_rect = Rect {
            x: 0,
            y: 3,
            height: area.height - 3,
            width: area.width,
        };

        Tabs::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Magenta)),
            )
            .select(state.active_tab)
            .style(
                tui::style::Style::default()
                    .fg(tui::style::Color::Cyan)
                    .bg(Color::Black),
            )
            .highlight_style(tui::style::Style::default().fg(tui::style::Color::LightCyan))
            .titles(&["Configuration", "Views", "Actions Log"])
            .divider("<->")
            .render(tab_bar_rect, buf);
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Magenta));
        let inner = block.inner(content_rect);
        block.render(content_rect, buf);
        match state.active_tab {
            0 => {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                    .split(inner);
                ActionsWidgetBuilder::default()
                    .reactor(&self.reactor)
                    .build()
                    .expect("Failed to build Actions Widget")
                    .render(chunks[0], buf);
                ConfigurationWidgetBuilder::default()
                    .config(self.config)
                    .build()
                    .expect("Failed to build Configuration Widget")
                    .render(chunks[1], buf);
            }
            1 => {
                ViewsDebugWidget::new(&self.editor, state).render(inner, buf);
            }
            2 => {
                ActionLogWidgetBuilder::default()
                    .actions(&state.actions)
                    .build()
                    .expect("Failed to build Action Log Widget")
                    .render(inner, buf);
            }
            _ => unreachable!(),
        }
    }
}
