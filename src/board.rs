use std::error::Error;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Board {
    pub cells: [[Cell; 9]; 9],
    pub solution: [[u8; 9]; 9],
    name: String,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Cell {
    pub digit: Option<u8>,
    pub fixed: bool,
}

impl Board {
    pub fn new() -> Self {
        Self::import().unwrap()
    }

    pub fn get(&self, x: usize, y: usize) -> &Cell {
        &self.cells[y][x]
    }

    pub fn set(&mut self, x: usize, y: usize, val: Option<u8>) {
        self.cells[y][x].digit = val;
    }
}

#[derive(Serialize, Deserialize)]
struct SudokuPuzzle {
    name: String,
    board: String,
    solution: String,
}

impl Board {
    pub fn import() -> Result<Self, Box<dyn Error>> {
        let puzzle: SudokuPuzzle = serde_json::from_str(include_str!("../assets/boards/basic.sudoku"))?;

        let cells = {
            let mut res = [[Cell::default(); 9]; 9];

            for (idx, c) in puzzle.board.chars().enumerate() {
                let y = idx % 9;
                let x = idx / 9;

                if c.is_numeric() {
                    res[y][x].digit = c.to_digit(10).map(|v| v as u8);
                    res[y][x].fixed = true;
                }
            }

            res
        };
        let solution = {
            let mut res = [[0; 9]; 9];

            for (idx, c) in puzzle.solution.chars().enumerate() {
                let y = idx % 9;
                let x = idx / 9;

                if c.is_numeric() {
                    res[y][x] = c.to_digit(10).map(|v| v as u8).unwrap();
                }
            }

            res
        };

        Ok(Self {
            name: puzzle.name,
            cells,
            solution,
        })
    }
}
