use anyhow::Result;
use crossterm::{
    cursor::{Hide, Show},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use log::{error, info, LevelFilter};
use std::io;

use crate::engine::main_loop;

mod engine;
mod life;
mod render;
mod menu;

fn main() -> Result<()> {
    simple_logging::log_to_file("command-life.log", LevelFilter::Debug)?;
    info!("Start app");

    // Setup terminal
    let mut std_out = io::stdout();
    terminal::enable_raw_mode()?;
    std_out.execute(EnterAlternateScreen)?;
    std_out.execute(Hide)?;

    if let Err(err) = main_loop() {
        error!("Main loop has crashed: {:#?}", err);
    }

    // Destroy terminal
    std_out.execute(Show)?;
    std_out.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
