use crate::{
    engine::{Ctx, State, StateMachine},
    menu::Menu,
    render::{draw_rec, draw_text, Cell, Frame},
};
use crossterm::{event::KeyCode, style::Color};
use rand::Rng;
use std::time::Duration;

#[derive(PartialEq)]
enum Status {
    Pause,
    Runing,
}

pub struct Life {
    current_cells: Vec<Vec<bool>>,
    next_cells: Vec<Vec<bool>>,
    alive_rule: [bool; 9],
    dead_rule: [bool; 9],
    born_rule: [bool; 9],
    alive_threshold: f32,
    rows: u32,
    cols: u32,
    step_time: Duration,
    status: Status,
    seeded: bool,
}

impl Life {
    fn fill_current(&mut self, ctx: &mut Ctx) {
        (0..self.current_cells.len()).for_each(|x| {
            (0..self.current_cells[0].len()).for_each(|y| {
                if ctx.rng.gen::<f32>() < self.alive_threshold {
                    self.current_cells[x][y] = true;
                }
            });
        });
    }
}

impl State for Life {
    fn input_handle(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char(' ') if self.status == Status::Runing => self.status = Status::Pause,
            KeyCode::Char(' ') if self.status == Status::Pause => self.status = Status::Runing,
            _ => {}
        }
    }

    fn update(&mut self, ctx: &mut Ctx) {
        if self.status == Status::Pause {
            return;
        }
        match self.step_time.saturating_sub(ctx.dt) {
            Duration::ZERO => {
                self.step_time = Duration::from_secs_f32(0.5);
            }
            _ => {
                self.step_time = self.step_time.saturating_sub(ctx.dt);
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
        if self.status == Status::Pause {
            draw_rec(
                2,
                2,
                7,
                3,
                Cell {
                    foreground_color: Color::White,
                    background_color: Color::Black,
                    char: ' ',
                },
                frame,
            );
            draw_text(
                3,
                3,
                "PAUSE",
                Cell {
                    foreground_color: Color::White,
                    background_color: Color::Black,
                    char: ' ',
                },
                frame,
            );
        }
    }
}

impl StateMachine<Life> {
    pub fn new(ctx: &mut Ctx) -> Self {
        let mut new_life = Self(Life {
            current_cells: vec![vec![false; ctx.rows as usize]; ctx.cols as usize],
            next_cells: vec![vec![false; ctx.rows as usize]; ctx.cols as usize],
            alive_rule: [false, false, true, true, false, false, false, false, false],
            dead_rule: [true, true, false, false, true, true, true, true, true],
            born_rule: [false, false, false, true, false, false, false, false, false],
            alive_threshold: 0.50,
            cols: ctx.cols as u32,
            rows: ctx.rows as u32,
            step_time: Duration::from_secs_f32(0.5),
            status: Status::Pause,
            seeded: true,
        });
        new_life.0.fill_current(ctx);
        new_life
    }
}

impl From<StateMachine<Menu>> for StateMachine<Life> {
    fn from(menu: StateMachine<Menu>) -> Self {
        Self(Life {
            current_cells: vec![
                vec![false; menu.0.selected_rows as usize];
                menu.0.selected_cols as usize
            ],
            next_cells: vec![
                vec![false; menu.0.selected_rows as usize];
                menu.0.selected_cols as usize
            ],
            alive_rule: [false, false, true, true, false, false, false, false, false],
            dead_rule: [true, true, false, false, true, true, true, true, true],
            born_rule: [false, false, false, true, false, false, false, false, false],
            alive_threshold: 0.50,
            cols: menu.0.selected_cols as u32,
            rows: menu.0.selected_rows as u32,
            step_time: Duration::from_secs_f32(0.5),
            status: Status::Pause,
            seeded: false,
        })
    }
}
