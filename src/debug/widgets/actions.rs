use crate::enki::ActionReactor;
use crossterm::event::Event;
use derive_builder::Builder;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, List, Text, Widget};

#[derive(Builder)]
pub struct ActionsWidget<'a> {
    pub reactor: &'a ActionReactor,
}

impl<'a> Widget for ActionsWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow))
            .title_style(Style::default().fg(Color::Red))
            .title(" Action Bindings ");
        block.render(area, buf);
        let inner = block.inner(area);

        let bindings = self.reactor.bindings();
        let iter = bindings.iter().map(|item| {
            let event = get_event_display_string(item.0);
            Text::raw(format!("{} = {:?}", event, item.1))
        });
        List::new(iter).render(inner, buf);
    }
}

fn get_event_display_string(event: &Event) -> String {
    match event {
        Event::Resize(_, _) => "Resize".into(),
        Event::Key(event) => format!("{:?} {:?}", event.code, event.modifiers),
        Event::Mouse(event) => format!("{:?}", event),
    }
}
