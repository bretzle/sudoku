use board::Board;
use macroquad::prelude::*;

mod board;

pub struct Sudoku {
    pub board: Board,
    selected_cell: Option<(f32, f32)>,
    hover_cell: Option<(f32, f32)>,
}

static DIGIT_TEXT: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

fn convert_pos(pos: (f32, f32)) -> (f32, f32) {
    (
        (pos.0 / Sudoku::SPACING).floor(),
        (pos.1 / Sudoku::SPACING).floor(),
    )
}

impl Sudoku {
    const SPACING: f32 = 100.0;

    pub fn new() -> Self {
        Self {
            board: Board::new(),
            selected_cell: Some((4.0, 4.0)),
            hover_cell: None,
        }
    }

    pub fn hover(&mut self, pos: (f32, f32)) {
        let (x, y) = convert_pos(pos);
        if x < 9.0 && y < 9.0 {
            self.hover_cell = Some((x, y));
        }
    }

    pub fn select(&mut self, pos: (f32, f32)) {
        let (x, y) = convert_pos(pos);
        if x < 9.0 && y < 9.0 {
            self.selected_cell = Some((x, y));
        }
    }

    pub fn render(&self) {
        self.draw_hover();
        self.draw_highlights();
        self.draw_board();
        self.draw_digits();
    }

    pub fn input(&mut self, val: Option<u8>) {
        if let Some(pos) = self.selected_cell {
            let x = pos.0 as usize;
            let y = pos.1 as usize;

            if !self.board.get(x, y).fixed {
                self.board.set(x, y, val);
            }
        }
    }
}

impl Sudoku {
    fn draw_hover(&self) {
        if let Some(cell) = self.hover_cell {
            draw_rectangle(
                cell.0 * Self::SPACING,
                cell.1 * Self::SPACING,
                Self::SPACING,
                Self::SPACING,
                Color::new(0.95, 0.95, 0.95, 1.0),
            );
        }
    }

    fn draw_highlights(&self) {
        // draw cells that are fixed
        for y in 0..9 {
            for x in 0..9 {
                let cell = self.board.get(x, y);
                if cell.fixed {
                    draw_rectangle(
                        (x as f32) * Self::SPACING,
                        (y as f32) * Self::SPACING,
                        Self::SPACING,
                        Self::SPACING,
                        Color::new(0.9, 0.9, 0.9, 1.0),
                    );
                }
            }
        }

        // highlight cells that share value with selected cell
        if let Some(ref cell) = self.selected_cell {
            if let Some(digit) = self.board.get(cell.0 as usize, cell.1 as usize).digit {
                for y in 0..9 {
                    for x in 0..9 {
                        if let Some(other_digit) = self.board.get(x, y).digit {
                            if other_digit == digit {
                                draw_rectangle(
                                    (x as f32) * Self::SPACING,
                                    (y as f32) * Self::SPACING,
                                    Self::SPACING,
                                    Self::SPACING,
                                    Color::new(0.8, 0.8, 0.9, 1.0),
                                );
                            }
                        }
                    }
                }
            }
        }

        // Draw selected cell
        if let Some(ref cell) = self.selected_cell {
            draw_rectangle(
                (cell.0 as f32) * Self::SPACING,
                (cell.1 as f32) * Self::SPACING,
                Self::SPACING,
                Self::SPACING,
                Color::new(0.8, 0.9, 0.8, 1.0),
            );
        }
    }

    fn draw_board(&self) {
        // Draw guides
        for n in 1..9 {
            let mut thick = 2.0;
            if n % 3 == 0 {
                thick = 8.0;
            }
            draw_rectangle(
                (n as f32) * Self::SPACING - thick / 2.0,
                0.0,
                thick / 2.0,
                Self::SPACING * 9.0,
                BLACK,
            );
            draw_rectangle(
                0.0,
                (n as f32) * Self::SPACING - thick / 2.0,
                Self::SPACING * 9.0,
                thick / 2.0,
                BLACK,
            );
        }

        draw_rectangle_lines(0.0, 0.0, 900.0, 900.0, 2.0, BLACK);
    }

    fn draw_digits(&self) {
        // Draw digits
        for y in 0..9 {
            for x in 0..9 {
                if let Some(digit) = self.board.cells[y][x].digit {
                    draw_text(
                        DIGIT_TEXT[digit as usize],
                        x as f32 * Self::SPACING + 30.0,
                        y as f32 * Self::SPACING + 75.0,
                        64.0,
                        BLACK,
                    )
                }
            }
        }
    }
}
