use crate::game::types::{PlayerSide, Square};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub side: PlayerSide,
    pub captured: Vec<Square>,
}

impl Player {
    pub fn new(name: String, side: PlayerSide) -> Self {
        let captured: Vec<Square> = vec![Square::Empty; 12];
        Self {name, side, captured}
    }

    pub fn collect_piece(&mut self, piece: Square) {
       if let Some(empty) = self.captured.iter_mut().rev().find(|sq| **sq == Square::Empty) {
            *empty = piece;
        } 
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WinningPlayer {
    None,
    Winner {player: Player},
}


impl WinningPlayer {
    pub fn set_winner(&mut self, winner: Player) -> Self {
        Self::Winner { player: winner }
    } 
}
