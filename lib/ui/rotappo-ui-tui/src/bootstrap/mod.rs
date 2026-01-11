use anyhow::Result;
use rotappo_ports::PortSet;

pub mod app;
pub mod panels;
pub mod state;
pub mod terminal;
pub mod utils;

use self::app::BootstrapApp;
use self::terminal::{TerminalGuard, run_app};

pub fn start_bootstrap(ports: PortSet) -> Result<()> {
    let mut terminal_guard = TerminalGuard::new()?;
    let mut app = BootstrapApp::new(ports);
    run_app(terminal_guard.terminal_mut(), &mut app)
}
