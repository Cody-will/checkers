use crate::game::board::Board;

#[derive(Debug, Clone, PartialEq,Eq)]
pub struct Position {
    pub row: i8,
    pub col: i8,
}

impl Position {
    pub fn new(row: i8, col: i8) -> Self {
        Self {row, col}
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Square {
    Empty,
    RedBase,
    RedKing,
    BlackBase,
    BlackKing,
}

impl Square {
    pub fn is_players(&self, side: &PlayerSide) -> bool {
        let get_side = match self {
            Square::BlackBase => PlayerSide::Black,
            Square::BlackKing => PlayerSide::Black,
            Square::RedBase => PlayerSide::Red,
            Square::RedKing => PlayerSide::Red,
            Square::Empty => return false,
        };
        side == &get_side
    }

    pub fn convert(&self) -> i32 {
        match self {
            Square::Empty => 0,
            Square::RedBase => 1,
            Square::RedKing => 2,
            Square::BlackBase => 3,
            Square::BlackKing => 4,
        }
    }

    pub fn is_empty(&self) -> bool {
        self == &Square::Empty
    }

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerSide {
    Red,
    Black,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameStatus {
    InProgess,
    Winning,
    Draw,
}

#[derive(Debug, Clone)]
pub struct PlayerMove {
    pub from: Position,
    pub to: Position,
}

impl PlayerMove {
    pub fn new(from: Position, to: Position) -> Self {
        Self {from, to}
    }
}

#[derive(Clone, Debug)]
pub struct SelectionState {
    pub selected: Option<Position>,
    pub targets: Vec<Position>,
}

#[derive(Debug, Clone)]
pub struct UiHints {
    selected: Option<Position>,
    selectable: Vec<Position>,
    targets: Vec<Position>,
    flat_board: Vec<i32>,
}

impl UiHints {
    pub fn new(board: &Board) -> Self {
       Self { selected: None, selectable: Vec::new(), targets: Vec::new(), flat_board: board.flatten()}  
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Direction {
    Up, // red side moving towards black side
    Down, // black side moving towards red side
}

impl Direction {
    pub fn current(side: &PlayerSide) -> Self {
        match side {
            PlayerSide::Black => Direction::Down,
            PlayerSide::Red => Direction::Up,
        }
    }
}



