use anyhow::Result;
use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use log::{debug, info, LevelFilter};
use rand::thread_rng;
use std::{
    io, thread,
    time::{Duration, Instant},
};

mod render;

#[derive(Debug)]
pub struct Ctx {
    rows: u16,
    cols: u16,
}

fn main() -> Result<()> {
    simple_logging::log_to_file("command-life.log", LevelFilter::Debug);
    info!("Start app");
    // Create random generator
    let mut rng = thread_rng();

    // Setup terminal
    let mut std_out = io::stdout();
    terminal::enable_raw_mode()?;
    std_out.execute(EnterAlternateScreen)?;
    std_out.execute(Hide)?;

    // Create context
    let (cols, rows) = terminal::size()?;
    let mut ctx = Ctx { cols, rows };
    debug!("Terminal detected: {:#?}", ctx);

    let mut frame_time = Instant::now();
    let mut dt = Duration::from_secs(0);

    'main_loop: loop {
        // debug!("Frame, dt: {:#?}", dt);
        // Update delta time
        dt = frame_time.elapsed();
        frame_time = Instant::now();

        while event::poll(Duration::from_secs(0))? {
            match event::read()? {
                Event::Key(key) => match key.code {
                    KeyCode::Esc | KeyCode::Char('q') => break 'main_loop,
                    _ => (),
                },
                Event::Resize(x, y) => {
                    debug!("Resize Event: {} {}", x, y);
                    ctx.cols = x;
                    ctx.rows = y;
                }
                _ => (),
            }
        }

        thread::sleep(Duration::from_millis(1));
    }

    // Destroy terminal
    std_out.execute(Show)?;
    std_out.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
