use crate::game::types::{Square, Position}; 


#[derive(Debug, Clone)]
pub struct Board {
    pub board: Vec<Vec<Square>>,
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    pub fn new() -> Self {
        let mut board = Vec::with_capacity(8);
        for row in 0..8 {
            let mut build_row = Vec::with_capacity(8); 
            for col in 0..8 {
                let is_dark = (row + col) % 2 == 1;

                let square = if is_dark && row < 3 {
                    Square::BlackBase
                } else if is_dark && row >= 5 {
                    Square::RedBase
                } else {
                    Square::Empty
                };

                build_row.push(square);
            }
            board.push(build_row);
        } 
        Self { board }
    }

    pub fn get_value(&self, position: &Position) -> Square {
        self.board[position.row as usize][position.col as usize]
    }

    pub fn flatten(&self) -> Vec<i32> {
        let mut out = Vec::with_capacity(64);
        for row in &self.board {
            for &square in row {
                out.push(square.convert());
            }
        }
        out
    } 
}
