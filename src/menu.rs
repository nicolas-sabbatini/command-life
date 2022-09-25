use crate::{
    engine::{Ctx, State},
    life::Life,
    render::{draw_rec, draw_text, Cell, Frame},
};
use crossterm::{event::KeyCode, style::Color};

#[derive(PartialEq)]
enum CursorPosition {
    Cols,
    Rows,
    Play,
}

impl CursorPosition {
    fn move_up(&self) -> Self {
        match self {
            CursorPosition::Cols => CursorPosition::Play,
            CursorPosition::Rows => CursorPosition::Cols,
            CursorPosition::Play => CursorPosition::Rows,
        }
    }

    fn move_down(&self) -> Self {
        match self {
            CursorPosition::Cols => CursorPosition::Rows,
            CursorPosition::Rows => CursorPosition::Play,
            CursorPosition::Play => CursorPosition::Cols,
        }
    }
}

#[non_exhaustive]
enum Transitions {
    No,
    Life,
}

pub struct Menu {
    pub min_cols: u16,
    pub min_rows: u16,
    pub max_cols: u16,
    pub max_rows: u16,
    pub selected_cols: u16,
    pub selected_rows: u16,
    cursor_position: CursorPosition,
    next_state: Transitions,
}

impl Menu {
    pub fn new(ctx: &mut Ctx) -> Self {
        Menu {
            min_cols: 10,
            min_rows: 10,
            max_cols: 999,
            max_rows: 999,
            selected_cols: ctx.cols,
            selected_rows: ctx.rows,
            cursor_position: CursorPosition::Cols,
            next_state: Transitions::No,
        }
    }

    fn decrease_value(&mut self) {
        match self.cursor_position {
            CursorPosition::Cols => {
                if self.selected_cols > self.min_cols {
                    self.selected_cols -= 1;
                }
            }
            CursorPosition::Rows => {
                if self.selected_rows > self.min_rows {
                    self.selected_rows -= 1;
                }
            }
            _ => {}
        }
    }

    fn increase_value(&mut self) {
        match self.cursor_position {
            CursorPosition::Cols => {
                if self.selected_cols < self.max_cols {
                    self.selected_cols += 1;
                }
            }
            CursorPosition::Rows => {
                if self.selected_rows < self.max_rows {
                    self.selected_rows += 1;
                }
            }
            _ => {}
        }
    }
}

impl State for Menu {
    fn input_handle(&mut self, key: KeyCode) {
        match key {
            KeyCode::Up => self.cursor_position = self.cursor_position.move_up(),
            KeyCode::Down => self.cursor_position = self.cursor_position.move_down(),
            KeyCode::Left => self.decrease_value(),
            KeyCode::Right => self.increase_value(),
            KeyCode::Char(' ') | KeyCode::Enter if self.cursor_position == CursorPosition::Play => {
                self.next_state = Transitions::Life
            }
            _ => (),
        }
    }

    fn update(&mut self, _ctx: &mut Ctx) {}

    fn draw(&self, frame: &mut Frame) {
        let cell = Cell {
            foreground_color: Color::White,
            background_color: Color::Black,
            char: ' ',
        };
        let selected_cell = Cell {
            foreground_color: Color::White,
            background_color: Color::DarkRed,
            char: ' ',
        };
        draw_rec(2, 2, 25, 8, cell, frame);
        draw_text(3, 2, "WELCOME TO COMMAND LIFE", cell, frame);
        let is_selected = |name: CursorPosition, current: &CursorPosition| {
            if name == *current {
                return selected_cell;
            }
            cell
        };

        draw_text(
            3,
            5,
            &format!("COLS            - {} +", self.selected_cols),
            is_selected(CursorPosition::Cols, &self.cursor_position),
            frame,
        );
        draw_text(
            3,
            6,
            &format!("ROWS            - {} +", self.selected_rows),
            is_selected(CursorPosition::Rows, &self.cursor_position),
            frame,
        );
        draw_text(
            3,
            8,
            "PLAY",
            is_selected(CursorPosition::Play, &self.cursor_position),
            frame,
        );
    }

    fn next(&self) -> Option<Box<dyn State>> {
        match self.next_state {
            Transitions::No => None,
            Transitions::Life => Some(Box::new(Life::from(self))),
            _ => None,
        }
    }
}
