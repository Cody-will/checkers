
// tests/combo.rs
mod common;
use common::*;

use checkers::game::play::Game;
use checkers::game::rules::ForcedMoveState;
use checkers::game::types::{Square, PlayerSide, PlayerMove};

#[test]
fn after_a_capture_if_another_jump_exists_turn_does_not_switch_and_state_is_continuingcombo() {
    let mut g = Game::new();
    g.board = empty_board();
    g.turn = PlayerSide::Red;

    // Create a 2-jump chain for Red:
    // Start: Red at (5,2)
    // Jump 1: over Black at (4,3) -> land (3,4)
    // Jump 2: over Black at (2,5) -> land (1,6)
    let start = pos(5, 2);
    let mid1 = pos(4, 3);
    let land1 = pos(3, 4);
    let mid2 = pos(2, 5);
    let land2 = pos(1, 6);

    set_sq(&mut g.board, start, Square::RedBase);
    set_sq(&mut g.board, mid1, Square::BlackBase);
    set_sq(&mut g.board, mid2, Square::BlackBase);
    // land squares empty

    // Select start so available_moves are filtered
    g.select(start);

    // First jump
    let jump1 = PlayerMove::new(start, land1);
    g.make_move(jump1);

    // Should still be Red's turn and forced into continuing combo from land1
    assert_eq!(g.turn, PlayerSide::Red, "turn should not switch during combo continuation");

    match g.available_moves.1 {
        ForcedMoveState::ContinuingCombo { from } => {
            assert_eq!(from, land1);
        }
        _ => panic!("expected ContinuingCombo after first jump when another jump exists"),
    }

    assert_eq!(g.selected, Some(land1), "selected should update to new position during combo");

    // Second jump should now be available and should finish combo and then switch turn
    let available = g.available_moves.0.clone();
    assert!(
        available.iter().any(|m| m.from == land1 && m.to == land2),
        "expected second jump to be offered from land1 to land2"
    );

    g.make_move(PlayerMove::new(land1, land2));
    assert_eq!(g.turn, PlayerSide::Black, "turn should switch after combo ends");
}

#[test]
fn capture_adds_piece_to_current_players_captured_and_removes_jumped_piece_from_board() {
    let mut g = Game::new();
    g.board = empty_board();
    g.turn = PlayerSide::Red;

    let start = pos(5, 2);
    let jumped = pos(4, 3);
    let land = pos(3, 4);

    set_sq(&mut g.board, start, Square::RedBase);
    set_sq(&mut g.board, jumped, Square::BlackBase);

    g.select(start);
    g.make_move(PlayerMove::new(start, land));

    // jumped square should now be empty
    assert_eq!(get_sq(&g.board, jumped), Square::Empty);

    // Player 1 is Red at game init; capture should fill the LAST empty slot in captured (because you .rev().find)
    let red_player = &g.players[0];
    assert_eq!(red_player.side, PlayerSide::Red);

    let last = red_player.captured.last().copied().unwrap();
    assert_eq!(last, Square::BlackBase, "expected captured piece stored in the last slot");
}
