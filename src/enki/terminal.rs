use tui::backend::CrosstermBackend;
use tui::terminal::Terminal as TuiTerminal;

use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event, EventStream};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use tokio::stream::StreamExt;

use std::io::Write;
use std::io::{stdout, Stdout};

pub struct Terminal {
    pub stream: EventStream,
    pub backend: TuiTerminal<CrosstermBackend<Stdout>>,
}

impl Terminal {
    pub fn new() -> crate::Result<Terminal> {
        let mut stdout = stdout();
        enable_raw_mode()?;
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = TuiTerminal::new(CrosstermBackend::new(stdout))?;
        let stream = EventStream::new();
        Ok(Terminal { backend, stream })
    }

    pub async fn next_event(&mut self) -> Option<Event> {
        if let Some(event) = self.stream.next().await {
            event.ok()
        } else {
            None
        }
    }

    pub fn size(&self) -> crate::Result<(u16, u16)> {
        Ok(crossterm::terminal::size()?)
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        execute!(stdout(), DisableMouseCapture, LeaveAlternateScreen)
            .expect("Failed to clean up terminal");
        disable_raw_mode().expect("Failed to disable raw terminal mode");
    }
}
