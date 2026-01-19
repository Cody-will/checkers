
// tests/common/mod.rs

use checkers::game::board::Board;
use checkers::game::types::{Position, Square, PlayerMove};

#[allow(dead_code)]
pub fn pos(r: i8, c: i8) -> Position {
    Position::new(r, c)
}

/// Make a board with all squares empty.
#[allow(dead_code)]
pub fn empty_board() -> Board {
    let mut b = Board::new();
    for r in 0..8 {
        for c in 0..8 {
            b.board[r][c] = Square::Empty;
        }
    }
    b
}

#[allow(dead_code)]
pub fn set_sq(b: &mut Board, p: Position, sq: Square) {
    let u = p.to_usize();
    b.board[u.row][u.col] = sq;
}

#[allow(dead_code)]
pub fn get_sq(b: &Board, p: Position) -> Square {
    b.get_value(&p)
}

/// Convenience: create a move.
#[allow(dead_code)]
pub fn mv(from: Position, to: Position) -> PlayerMove {
    PlayerMove::new(from, to)
}

/// Convenience: dark-square check (matches your init logic).
#[allow(dead_code)]
pub fn is_dark(p: Position) -> bool {
    ((p.row + p.col) % 2) == 1
}

/// Count squares matching a predicate.
#[allow(dead_code)]
pub fn count_squares(b: &Board, f: impl Fn(Square) -> bool) -> usize {
    let mut n = 0usize;
    for r in 0..8 {
        for c in 0..8 {
            if f(b.board[r][c]) {
                n += 1;
            }
        }
    }
    n
}

#[allow(dead_code)]
pub fn assert_forced_state_none<T: core::fmt::Debug>(_state: &T) {
    // Kept as a helper hook in case you expand pattern matches later.
    // (We pattern match in individual tests where needed.)
}

