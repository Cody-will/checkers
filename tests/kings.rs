
// tests/kings.rs
mod common;
use common::*;

use checkers::game::play::Game;
use checkers::game::types::{Square, PlayerSide, PlayerMove};

#[test]
fn make_move_kings_piece_when_reaching_row_0_or_row_7() {
    let mut g = Game::new();

    // Clear board and set a red base near promotion
    g.board = empty_board();
    g.turn = PlayerSide::Red;

    let from = pos(1, 2);
    let to = pos(0, 1);
    set_sq(&mut g.board, from, Square::RedBase);

    // Must select to filter available_moves to this piece
    g.select(from);
    g.make_move(PlayerMove::new(from, to));

    assert_eq!(get_sq(&g.board, to), Square::RedKing, "expected RedBase to become RedKing on row 0");
}

#[test]
fn king_does_not_change_if_already_king() {
    let mut g = Game::new();
    g.board = empty_board();
    g.turn = PlayerSide::Red;

    let from = pos(1, 2);
    let to = pos(0, 1);
    set_sq(&mut g.board, from, Square::RedKing);

    g.select(from);
    g.make_move(PlayerMove::new(from, to));

    assert_eq!(get_sq(&g.board, to), Square::RedKing);
}
