use crate::game::board::Board;
use crate::game::play::Game;
use crate::game::player::Player;



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub row: i8,
    pub col: i8,
}

impl Position {
    pub fn new(row: i8, col: i8) -> Self {
        Self {row, col}
    }

    pub fn to_usize(self) -> PositionUsize {
        PositionUsize { row: self.row as usize, col: self.col as usize } 
    }
 
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PositionUsize {
    pub row: usize,
    pub col: usize,
}

impl PositionUsize {
    pub fn new(row: usize, col: usize) -> Self {
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
    pub fn is_players(&self, player_side: &PlayerSide) -> bool {
        let side = match self {
            Square::BlackBase => PlayerSide::Black,
            Square::BlackKing => PlayerSide::Black,
            Square::RedBase => PlayerSide::Red,
            Square::RedKing => PlayerSide::Red,
            Square::Empty => return false,
        };
        player_side == &side
    }

    pub fn is_opponents(&self, player_side: &PlayerSide) -> bool {
        let side = match self {
            Self::BlackBase => PlayerSide::Black,
            Self::BlackKing => PlayerSide::Black,
            Self::RedBase => PlayerSide::Red,
            Self::RedKing => PlayerSide::Red,
            Self::Empty => return true,
        };
        player_side != &side
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

    pub fn to_king(self) -> Self {
        match self {
            Self::RedBase => Self::RedKing,
            Self::BlackBase => Self::BlackKing,
            Self::BlackKing => Self::BlackKing,
            Self::RedKing => Self::RedKing,
            Self::Empty => Self::Empty,

        }
    }

    pub fn is_king(self) -> bool {
        matches!(self, Self::BlackKing | Self::RedKing)    
    }

    pub fn is_empty(&self) -> bool {
        self == &Square::Empty
    }

}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerSide {
    Red,
    Black,
}

impl PlayerSide {
    pub fn switch(self) -> Self {
        match self {
            Self::Red => Self::Black,
            Self::Black => Self::Red,
        }
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameStatus {
    InProgess,
    Winning,
    Draw,
}



#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlayerMove {
    pub from: Position,
    pub to: Position,
}

impl PlayerMove {
    pub fn new(from: Position, to: Position) -> Self {
        Self {from, to}
    }

    pub fn apply_move(&self, board: &mut Board) -> Position {
        let PlayerMove {from, to} = self;
        let from_u = from.to_usize();
        let to_u = to.to_usize();
        let  board = &mut board.board;
        let piece = board[from_u.row][from_u.col];
        board[from_u.row][from_u.col] = Square::Empty;
        board[to_u.row][to_u.col] = piece;
        Position::new(to.row, to.col)
    }

    
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlintUi {
    pub board: Vec<i32>,
    pub selected: i32,
    pub selectable: Vec<bool>,
    pub targets: Vec<bool>,
    pub players: [Player; 2],
    pub turn: PlayerSide,
}

impl SlintUi {
    pub fn new(hints: &UiHints) -> Self {
        let board = hints.board.clone();
        let selected: i32 = match hints.selected {
            Some(sel) => {let Position {row, col} = sel; (row * 8 + col) as i32},
            None => {-1},
        };
        let (selectable, targets) = hints.flatten_available();
        let players = hints.players.clone();
        let turn = hints.turn;
        Self { board, selected, selectable, targets, players, turn }
    }
}


#[derive(Debug, Clone)]
pub struct UiHints {
    selected: Option<Position>,
    available: Vec<PlayerMove>,
    board: Vec<i32>,
    players: [Player; 2],
    turn: PlayerSide,
}

impl UiHints {
    pub fn new(game: &Game) -> Self {
        let selected = game.selected;
        let available = game.available_moves.0.clone();  
        let players = game.players.clone();
        let turn = game.turn;

        Self { selected, available, board: game.board.flatten(), players, turn }  
    } 

    pub fn flatten_ui(&self) -> SlintUi { 
        SlintUi::new(&self)
    }

    fn flatten_available(&self) -> (Vec<bool>, Vec<bool>) {
        let mut selectable: Vec<bool> = vec![false; 64];
        let mut targets: Vec<bool>  = vec![false; 64];   
        for mv in &self.available {
            let from = mv.from;
            let index = (from.row * 8 + from.col) as usize;
            selectable[index] = true;
        }

        if let Some(sel) = self.selected {
            let selected = (sel.row * 8 + sel.col) as usize;
            for mv in &self.available {
                let from = (mv.from.row * 8 + mv.from.col) as usize;
                if from == selected {
                    let to = (mv.to.row * 8 + mv.to.col) as usize;
                    targets[to] = true;
                }
            }
        }


        (selectable, targets)
    }
}




#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up, // red side moving towards black side
    Down, // black side moving towards red side
}

impl Direction {
    pub fn current(side: &PlayerSide) -> Self {
        match side {
            PlayerSide::Black => Self::Down,
            PlayerSide::Red => Self::Up,
        }
    }
}



