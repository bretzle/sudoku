use macroquad::prelude::*;

pub struct Sudoku {
    board: Board,
    selected_cell: Option<(f32, f32)>,
    hover_cell: Option<(f32, f32)>,
}

static DIGIT_TEXT: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

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
        self.hover_cell = Some((
            (pos.0 / Sudoku::SPACING as f32).floor(),
            (pos.1 / Sudoku::SPACING as f32).floor(),
        ));
    }

    pub fn select(&mut self, pos: (f32, f32)) {
        self.selected_cell = Some((
            (pos.0 / Sudoku::SPACING as f32).floor(),
            (pos.1 / Sudoku::SPACING as f32).floor(),
        ));
    }

    pub fn render(&self) {
        self.draw_board();
    }

    fn draw_board(&self) {
        // hover cell
        if let Some(cell) = self.hover_cell {
            draw_rectangle(
                cell.0 * Self::SPACING,
                cell.1 * Self::SPACING,
                Self::SPACING,
                Self::SPACING,
                Color::new(0.95, 0.95, 0.95, 1.0),
            );
        }

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
                screen_height(),
                Color::new(0.0, 0.0, 0.0, 1.0),
            );
            draw_rectangle(
                0.0,
                (n as f32) * Self::SPACING - thick / 2.0,
                screen_width(),
                thick / 2.0,
                Color::new(0.0, 0.0, 0.0, 1.0),
            );
        }
    }

    pub fn input(&mut self, val: u8) {
        if let Some(pos) = self.selected_cell {
            self.board.set(pos.0 as usize, pos.1 as usize, val);
        }
    }
}

struct Board {
    cells: [[Cell; 9]; 9],
}

#[derive(Clone, Copy)]
struct Cell {
    pub digit: Option<u8>,
    pub fixed: bool,
}

impl Board {
    fn new() -> Self {
        let mut cells = [[Cell {
            digit: None,
            fixed: false,
        }; 9]; 9];

        cells[2][2] = Cell {
            digit: Some(5),
            fixed: true,
        };
        cells[4][4] = Cell {
            digit: Some(5),
            fixed: true,
        };

        Self { cells }
    }

    fn get(&self, x: usize, y: usize) -> &Cell {
        &self.cells[y][x]
    }

    fn set(&mut self, x: usize, y: usize, val: u8) {
        self.cells[y][x].digit = Some(val);
    }
}
