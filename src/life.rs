use std::time::Duration;

use crate::{
    engine::{Ctx, State, StateMachine},
    render::{Cell, Frame},
};
use crossterm::{event::KeyCode, style::Color};
use rand::Rng;

pub struct Life {
    current_cells: Vec<Vec<bool>>,
    next_cells: Vec<Vec<bool>>,
    alive_rule: [bool; 9],
    dead_rule: [bool; 9],
    born_rule: [bool; 9],
    rows: u32,
    cols: u32,
    step_time: Duration,
    runing: bool,
}

impl State for Life {
    fn input_handle(&mut self, key: KeyCode) {
        if let KeyCode::Char(' ') = key {
            self.runing = !self.runing
        }
    }

    fn update(&mut self, dt: &Duration) {
        if !self.runing {
            return;
        }
        match self.step_time.saturating_sub(*dt) {
            Duration::ZERO => {
                self.step_time = Duration::from_secs_f32(0.5);
            }
            _ => {
                self.step_time = self.step_time.saturating_sub(*dt);
                return;
            }
        }

        self.current_cells.iter().enumerate().for_each(|(x, col)| {
            col.iter().enumerate().for_each(|(y, cell)| {
                let mut neighbours = -(*cell as i32);
                let c_x = x as i32;
                let c_y = y as i32;
                for off_x in -1..=1 {
                    for off_y in -1..=1 {
                        let n_x = c_x + off_x;
                        let n_y = c_y + off_y;
                        if n_x >= 0 && n_x < self.cols as i32 && n_y >= 0 && n_y < self.rows as i32
                        {
                            neighbours += self.current_cells[n_x as usize][n_y as usize] as i32
                        }
                    }
                }
                if (*cell && self.alive_rule[neighbours as usize])
                    || (!(*cell) && self.born_rule[neighbours as usize])
                {
                    self.next_cells[x][y] = true;
                } else if *cell && self.dead_rule[neighbours as usize] {
                    self.next_cells[x][y] = false;
                } else {
                    self.next_cells[x][y] = *cell;
                }
            })
        });
        std::mem::swap(&mut self.next_cells, &mut self.current_cells);
    }

    fn draw(&self, frame: &mut Frame) {
        let frame_width = frame.len();
        let frame_height = frame[0].len();
        self.current_cells.iter().enumerate().for_each(|(x, col)| {
            col.iter().enumerate().for_each(|(y, cell)| {
                if x < frame_width && y < frame_height {
                    frame[x][y] = Cell {
                        foreground_color: Color::White,
                        background_color: Color::DarkGrey,
                        char: ' ',
                    };
                    if *cell {
                        frame[x][y].char = 'X';
                    }
                }
            })
        });
    }
}

impl StateMachine<Life> {
    pub fn new(ctx: &mut Ctx) -> Self {
        let mut current_cells = vec![vec![false; ctx.rows as usize]; ctx.cols as usize];
        for x in 0..ctx.cols as usize {
            for y in 0..ctx.rows as usize {
                if ctx.rng.gen::<f32>() < 0.49 {
                    current_cells[x][y] = true;
                }
            }
        }
        Self(Life {
            current_cells,
            next_cells: vec![vec![false; ctx.rows as usize]; ctx.cols as usize],
            alive_rule: [false, false, true, true, false, false, false, false, false],
            dead_rule: [true, true, false, false, true, true, true, true, true],
            born_rule: [false, false, false, true, false, false, false, false, false],
            cols: ctx.cols as u32,
            rows: ctx.rows as u32,
            step_time: Duration::from_secs_f32(0.5),
            runing: true,
        })
    }
}
