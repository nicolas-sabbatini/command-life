use anyhow::Result;
use crossterm::{
    cursor::MoveTo,
    style::{Color, SetBackgroundColor},
    terminal::{Clear, ClearType},
    QueueableCommand,
};
use std::io::{Stdout, Write};

use crate::Ctx;

pub type Frame = Vec<Vec<&'static str>>;
pub fn new_frame(ctx: &Ctx) -> Frame {
    vec![vec![" "; ctx.rows as usize]; ctx.cols as usize]
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}

pub fn draw(
    std_out: &mut Stdout,
    prev_frame: &Frame,
    new_frame: &Frame,
    force: bool,
) -> Result<()> {
    if force {
        std_out
            .queue(SetBackgroundColor(Color::Blue))?
            .queue(Clear(ClearType::All))?;
    }
    new_frame.iter().enumerate().for_each(|(x, col)| {
        col.iter().enumerate().for_each(|(y, char)| {
            if force || *char != prev_frame[x][y] {
                std_out
                    .queue(MoveTo(x as u16, y as u16 + 1))
                    .expect("Can't queue mouse movement");
                print!("{}", *char);
            }
        })
    });
    std_out.flush()?;
    Ok(())
}
