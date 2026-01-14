use crate::game::board::Board;
use crate::game::player::{ Player, WinningPlayer};
use crate::game::types::{GameStatus, PlayerMove, PlayerSide, Position, PositionUsize, Square, UiHints};
use crate::game::rules::{ForcedMoveState, get_moves, get_jumps};
use crate::AppWindow;
use crate::FlatUi;


#[derive(Debug, Clone)]
pub struct Game {
    pub board: Board,
    pub selected: Option<Position>,
    pub players: [Player; 2],
    pub winner: WinningPlayer,
    pub turn: PlayerSide,
    pub forced: ForcedMoveState,
    pub status: GameStatus,
    pub available_moves: (Vec<PlayerMove>, ForcedMoveState)

}

impl Default for Game {
    fn default() -> Self {
        let board: Board = Board::new();
        let selected = None;
        let player1 = Player::new(String::from("Player 1"), PlayerSide::Red);
        let player2 = Player::new(String::from("Player 2"), PlayerSide::Black);
        let players: [Player; 2] = [player1, player2];
        let turn = PlayerSide::Red;
        let available_moves = get_moves(&board, &turn); 
        Self { board, selected, players, winner: WinningPlayer::None, turn, forced: ForcedMoveState::None, status: GameStatus::InProgess, available_moves }
        
    }
}

impl Game {
    pub fn new() -> Self {
       Self::default() 
        
    }

    pub fn select(&mut self, pos: Position) {
        self.selected = Some(pos);
 
        let (all_moves, forced_state) = get_moves(&self.board, &self.turn);

        let filtered: Vec<PlayerMove> = all_moves
            .into_iter()
            .filter(|m| m.from == pos)
            .collect();

        self.available_moves = (filtered, forced_state);
    }

    pub fn sync_ui(&self, window: &AppWindow) {
        let ui = UiHints::new(self).flatten_ui(); 
        window.set_ui_hints(FlatUi::new(&ui)); 
    } 

     

    pub fn make_move(&mut self, player_move: PlayerMove) {
        let(available, forced_state) = &mut self.available_moves;
        
        match forced_state {
            ForcedMoveState::None => { 
                if !available.contains(&player_move) { return; }

                let new_position = player_move.apply_move(&mut self.board);    
                if new_position.row == 0 || new_position.row == 7 {self.make_king(&new_position);}  
                let is_winner = self.check_win();
                if is_winner {self.handle_win();}

                self.turn = self.turn.switch();
                self.available_moves = get_moves(&self.board, &self.turn);
                self.selected = None;
                
            },
            ForcedMoveState::MustCapture { from } => {
                if !from.contains(&player_move.from) { return; }

                let new_position = player_move.apply_move(&mut self.board);
                if new_position.row == 0 || new_position.row == 7 {self.make_king(&new_position);}
                let jumped_piece = self.get_jumped(&player_move);
                let curr_player = if self.players[0].side == self.turn {&mut self.players[0]} else {&mut self.players[1]};

                curr_player.collect_piece(self.board.get_value(&jumped_piece));
                self.remove_piece(&jumped_piece);
                
                let is_winner = self.check_win();

                let jumps = get_jumps(&new_position, &self.board, &self.turn);

                if !jumps.is_empty() {
                    self.selected = Some(new_position);
                    self.available_moves = (jumps, ForcedMoveState::ContinuingCombo { from: new_position });
                } else {
                    self.turn = self.turn.switch();
                    self.available_moves = get_moves(&self.board, &self.turn);
                    self.selected = None;
                }

            }, 
            ForcedMoveState::ContinuingCombo { from } => {
                if *from != player_move.from { return; }

                let new_position = player_move.apply_move(&mut self.board);
                if new_position.row == 0 || new_position.row == 7 {self.make_king(&new_position);}
                let jumped_piece = self.get_jumped(&player_move);
                let curr_player = if self.players[0].side == self.turn {&mut self.players[0]} else {&mut self.players[1]};

                curr_player.collect_piece(self.board.get_value(&jumped_piece));
                self.remove_piece(&jumped_piece);

                let is_winner = self.check_win();

                let jumps = get_jumps(&new_position, &self.board, &self.turn);
                
                if !jumps.is_empty() {
                    self.selected = Some(new_position);
                    self.available_moves = (jumps, ForcedMoveState::ContinuingCombo { from: new_position });
                } else {
                    self.turn = self.turn.switch();
                    self.available_moves = get_moves(&self.board, & self.turn);
                    self.selected = None;
                }
            }
        }
    }
 
    
    fn make_king(&mut self, position: &Position) {
        let PositionUsize {row, col} = position.to_usize();
        let piece: Square = self.board.board[row][col];
        let king = piece.to_king();
        self.board.board[row][col] = king;
    }

    fn get_jumped(&self, player_move: &PlayerMove) -> Position {
        let PlayerMove {from, to} = player_move;
        let r = to.row - from.row;
        let c = to.col - from.col;
        Position::new(from.row + r / 2, from.col + c / 2)
    }

    fn remove_piece(&mut self, position: &Position) {
        let PositionUsize {row, col} = position.to_usize();
        self.board.board[row][col] = Square::Empty;
    }

    fn check_win(&self) -> bool {
        let mut win = true;
        let opponent = self.turn.switch();
        let available_moves = get_moves(&self.board, &opponent);
        if available_moves.0.is_empty() { return win; }
        for row in 0..8 {
            for col in 0..8 {
                let square = self.board.board[row][col];
                if square.is_opponents(&self.turn) { win = false; break; }
            }
            if !win {break;}
        }
        win
    }

    fn handle_win(&mut self) {
        self.status = GameStatus::Winning;
    }
 
}


