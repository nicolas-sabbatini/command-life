use crate::{
    life::Life,
    render::{draw, new_frame, Frame},
};
use anyhow::Result;
use crossbeam::channel::unbounded;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal,
};
use log::{debug, error, info};
use rand::{prelude::ThreadRng, thread_rng};
use std::{
    io, thread,
    time::{Duration, Instant},
};

pub trait State {
    fn input_handle(&mut self, key: KeyCode);
    fn update(&mut self, dt: &Duration);
    fn draw(&self, frame: &mut Frame);
}

#[derive(Debug)]
pub struct StateMachine<S: State>(pub S);

#[derive(Debug)]
pub struct Ctx {
    pub rows: u16,
    pub cols: u16,
    pub dt: Duration,
    pub rng: ThreadRng,
}

pub fn main_loop() -> Result<()> {
    // Create context
    let (cols, rows) = terminal::size()?;
    let mut ctx = Ctx {
        cols,
        rows,
        dt: Duration::from_secs(0),
        rng: thread_rng(),
    };
    info!("Terminal detected: {:#?}", ctx);

    // Create render
    let mut prev_frame = new_frame(&ctx);
    let (render_sender, render_reciver) = unbounded::<(Frame, bool)>();
    let render_thread = thread::spawn(move || {
        let mut std_out = io::stdout();
        draw(&mut std_out, &prev_frame, &prev_frame, true).expect("Error in frame buffer");
        while let Ok((new_frame, force)) = render_reciver.recv() {
            draw(&mut std_out, &prev_frame, &new_frame, force).expect("Error in frame buffer");
            prev_frame = new_frame;
        }
    });
    debug!("Render created");

    // Create start state
    let mut state_machine = StateMachine::<Life>::new(&mut ctx);

    let mut frame_time = Instant::now();
    'main_loop: loop {
        // debug!("Frame, dt: {:#?}", ctx.dt);
        // Update delta time
        ctx.dt = frame_time.elapsed();
        frame_time = Instant::now();

        let mut force = false;
        while event::poll(Duration::from_secs(0))? {
            match event::read()? {
                Event::Key(key) => match key.code {
                    KeyCode::Esc | KeyCode::Char('q') => break 'main_loop,
                    k => state_machine.0.input_handle(k),
                },
                Event::Resize(x, y) => {
                    debug!("Resize Event: {} {}", x, y);
                    ctx.cols = x;
                    ctx.rows = y;
                    force = true;
                }
                _ => (),
            }
        }

        state_machine.0.update(&ctx.dt);
        let mut frame = new_frame(&ctx);
        state_machine.0.draw(&mut frame);

        if let Err(err) = render_sender.send((frame, force)) {
            error!("Failed to sed frame to render {:#?}", err);
        };
        thread::sleep(Duration::from_millis(1));
    }
    // Kill channel
    drop(render_sender);
    // Await thread
    render_thread.join().unwrap();
    Ok(())
}
