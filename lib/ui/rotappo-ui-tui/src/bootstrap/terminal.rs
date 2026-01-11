use crate::bootstrap::app::BootstrapApp;
use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CrosstermEvent},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::Terminal;
use ratatui::backend::{Backend, CrosstermBackend};
use std::io::{self, Stdout};
use std::time::Duration;

const TICK_RATE: Duration = Duration::from_millis(200);

pub struct TerminalGuard {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl TerminalGuard {
    pub fn new() -> Result<Self> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;
        Ok(Self { terminal })
    }

    pub fn terminal_mut(&mut self) -> &mut Terminal<CrosstermBackend<Stdout>> {
        &mut self.terminal
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        );
        let _ = self.terminal.show_cursor();
    }
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut BootstrapApp) -> Result<()> {
    loop {
        terminal.draw(|frame| app.render(frame))?;
        if app.should_quit {
            break;
        }
        if event::poll(TICK_RATE)? {
            if let CrosstermEvent::Key(key) = event::read()? {
                app.handle_key_event(key)?;
            }
        }
        app.on_tick();
    }
    Ok(())
}
