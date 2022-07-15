use anyhow::Result;
use crossterm::{
    cursor::MoveTo,
    style::{Color, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    QueueableCommand,
};
use std::{
    io::{Stdout, Write},
    usize,
};

use crate::engine::Ctx;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Cell {
    pub foreground_color: Color,
    pub background_color: Color,
    pub char: char,
}
impl Default for Cell {
    fn default() -> Self {
        Cell {
            foreground_color: Color::Black,
            background_color: Color::Blue,
            char: ' ',
        }
    }
}

pub type Frame = Vec<Vec<Cell>>;
pub fn new_frame(ctx: &Ctx) -> Frame {
    vec![vec![Cell::default(); ctx.rows as usize]; ctx.cols as usize]
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
        col.iter().enumerate().for_each(|(y, cell)| {
            if force || *cell != prev_frame[x][y] {
                std_out
                    .queue(MoveTo(x as u16, y as u16 + 1))
                    .expect("Can't queue mouse movement")
                    .queue(SetBackgroundColor(cell.background_color))
                    .expect("Can't queue background color")
                    .queue(SetForegroundColor(cell.foreground_color))
                    .expect("Can't queue foreground color");
                print!("{}", cell.char);
            }
        })
    });
    std_out.flush()?;
    Ok(())
}

pub fn draw_rec(x: usize, y: usize, w: usize, h: usize, clear_cel: Cell, frame: &mut Frame) {
    for x_index in x..w + x {
        for y_index in y..y + h {
            if x_index < frame.len() && y_index < frame[0].len() {
                frame[x_index][y_index] = clear_cel;
            }
        }
    }
}

pub fn draw_text(x: usize, y: usize, text: &str, cel: Cell, frame: &mut Frame) {
    for (x_offset, c) in text.chars().enumerate() {
        if x + x_offset < frame.len() && y < frame.len() {
            let mut c_cell = cel;
            c_cell.char = c;
            frame[x + x_offset][y] = c_cell;
        }
    }
}
