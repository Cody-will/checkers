use crate::game::types::{ PlayerSide, SlintUi, Square};
use crate::game::player::Player;
use slint::{ModelRc, VecModel};

#[cfg(feature = "ui")]
use crate::{FlatUi, SlintPlayerSide, SlintSquare, SlintPlayer};


impl SlintPlayer {
    pub fn new(player: &Player) -> Self {
        let name = slint::SharedString::from(player.name.clone());
        let side = SlintPlayerSide::new(&player.side);
        let converted_captured: Vec<SlintSquare> = player.captured.clone().iter().map(SlintSquare::new).collect();
        let captured = ModelRc::new(VecModel::from(converted_captured));
        Self {name, side, captured}
    }
}

impl SlintSquare {
    pub fn new(square: &Square) -> Self {
        match square {
            Square::RedBase => Self::RedBase,
            Square::RedKing => Self::RedKing,
            Square::BlackBase => Self::BlackBase,
            Square::BlackKing => Self::BlackKing,
            Square::Empty => Self::Empty, 
        }
    }
}

impl SlintPlayerSide {
    pub fn new(side: &PlayerSide) -> Self {
        match side {
            PlayerSide::Red => Self::Red,
            PlayerSide::Black => Self::Black,
        }
    }
}


impl FlatUi {
    pub fn new(hints: &SlintUi) -> Self {
        let board = ModelRc::new(VecModel::from(hints.board.clone()));
        let selected = hints.selected;
        let selectable = ModelRc::new(VecModel::from(hints.selectable.clone()));
        let targets = ModelRc::new(VecModel::from(hints.targets.clone()));
        let players_copy = hints.players.clone();
        let players_converted: Vec<SlintPlayer> = players_copy.iter().map(SlintPlayer::new).collect();
        let players = ModelRc::new(VecModel::from(players_converted));
        let turn = SlintPlayerSide::new(&hints.turn);
     
        Self {board, selected, selectable, targets, players, turn}
    }
}
