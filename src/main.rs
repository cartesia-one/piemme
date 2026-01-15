//! Piemme - A TUI Prompt Manager
//!
//! A terminal-based user interface application for managing, organizing,
//! and composing reusable prompts with vim-like keybindings.

mod app;
mod config;
mod engine;
mod error;
mod fs;
mod models;
mod tui;
mod ui;

use anyhow::Result;
use app::App;

fn main() -> Result<()> {
    // Set up panic handler to restore terminal on crash
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        // Restore terminal before printing panic info
        let _ = tui::restore_terminal();
        original_hook(panic_info);
    }));

    // Initialize and run the application
    let mut app = App::new()?;
    app.run()?;

    Ok(())
}
