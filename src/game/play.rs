use crate::game::board::Board;
use crate::game::player::Player;
use crate::game::types::{PlayerSide, GameStatus, UiHints};
use crate::game::rules::ForcedMoveState;

#[derive(Debug, Clone)]
pub struct Game {
    pub board: Board,
    pub players: [Player; 2],
    pub turn: PlayerSide,
    pub forced: ForcedMoveState,
    pub status: GameStatus,
    pub ui: UiHints,
}

impl Game {
    pub fn new() -> Self {
        let board: Board = Board::new();
        let player1 = Player::new(String::from("Player 1"), PlayerSide::Red);
        let player2 = Player::new(String::from("Player 2"), PlayerSide::Black);
        let players: [Player; 2] = [player1, player2];
        let ui: UiHints = UiHints::new(&board);
        Self {board, players, turn: PlayerSide::Red, forced: ForcedMoveState::None, status: GameStatus::InProgess, ui}
    }

    pub fn start(&self) {
         
    }

    pub fn set_forced(&mut self, forced_state: ForcedMoveState) {
        self.forced = forced_state;
    }
 
}


