// tests/moves.rs
mod common;
use common::*;

use checkers::Board;
use checkers::game::rules::{ForcedMoveState, get_moves, get_jumps};
use checkers::game::types::{ Square, PlayerSide};

#[test]
fn board_new_has_12_red_12_black_on_dark_squares() {
    let b = Board::new();

    let red = count_squares(&b, |s| matches!(s, Square::RedBase | Square::RedKing));
    let black = count_squares(&b, |s| matches!(s, Square::BlackBase | Square::BlackKing));
    let empty = count_squares(&b, |s| s == Square::Empty);

    assert_eq!(red, 12);
    assert_eq!(black, 12);
    assert_eq!(empty, 40);

    // All light squares should be empty in initial setup
    for r in 0..8 {
        for c in 0..8 {
            let p = pos(r, c);
            let sq = b.get_value(&p);
            if !is_dark(p) {
                assert_eq!(sq, Square::Empty, "light square should be empty at {r},{c}");
            }
        }
    }
}

#[test]
fn flatten_returns_64_and_matches_convert_mapping() {
    let b = Board::new();
    let flat = b.flatten();

    assert_eq!(flat.len(), 64);

    // Spot-check a couple known squares in initial board:
    // (0,1) is dark and row < 3 => BlackBase => 3
    assert_eq!(flat[(0 * 8 + 1) as usize], Square::BlackBase.convert());
    // (7,0) is dark and row >= 5 => RedBase => 1
    assert_eq!(flat[(7 * 8 + 0) as usize], Square::RedBase.convert());
}

#[test]
fn initial_red_has_7_slide_moves_and_no_forced_capture() {
    let b = Board::new();
    let (moves, forced) = get_moves(&b, &PlayerSide::Red);

    match forced {
        ForcedMoveState::None => {}
        _ => panic!("expected ForcedMoveState::None at game start for Red"),
    }

    // Reasoning based on your init layout + slide rules:
    // Only row 5 red pieces can slide (row 6/7 are blocked). That yields 7 slides total.
    assert_eq!(moves.len(), 7, "expected 7 opening slide moves for Red");
}

#[test]
fn get_slides_does_not_generate_out_of_bounds_moves() {
    let mut b = empty_board();
    // Put a red man near top edge. Red moves "Up" (row - 1), so from row 0 should have 0 slides.
    set_sq(&mut b, pos(0, 1), Square::RedBase);

    let (moves, forced) = get_moves(&b, &PlayerSide::Red);
    assert!(moves.is_empty());
    matches!(forced, ForcedMoveState::None);
}

#[test]
fn get_jumps_requires_mid_is_opponent_and_landing_is_empty() {
    let mut b = empty_board();

    // Red at (4,3), black at (3,2), landing at (2,1)
    set_sq(&mut b, pos(4, 3), Square::RedBase);
    set_sq(&mut b, pos(3, 2), Square::BlackBase);
    // landing empty by default

    let jumps = get_jumps(&pos(4, 3), &b, &PlayerSide::Red);
    assert_eq!(jumps.len(), 1);
    assert_eq!(jumps[0].from, pos(4, 3));
    assert_eq!(jumps[0].to, pos(2, 1));

    // Block landing => no jump
    set_sq(&mut b, pos(2, 1), Square::RedBase);
    let jumps2 = get_jumps(&pos(4, 3), &b, &PlayerSide::Red);
    assert!(jumps2.is_empty());

    // Clear landing, but mid is same side => no jump
    set_sq(&mut b, pos(2, 1), Square::Empty);
    set_sq(&mut b, pos(3, 2), Square::RedBase);
    let jumps3 = get_jumps(&pos(4, 3), &b, &PlayerSide::Red);
    assert!(jumps3.is_empty());
}

#[test]
fn king_has_four_diagonal_slide_directions() {
    let mut b = empty_board();
    // Place a RedKing in the center
    set_sq(&mut b, pos(3, 3), Square::RedKing);

    let (moves, forced) = get_moves(&b, &PlayerSide::Red);
    match forced {
        ForcedMoveState::None => {}
        _ => panic!("king-only board should not force capture"),
    }

    // From (3,3), all 4 diagonals are in-bounds and empty => 4 slide moves
    assert_eq!(moves.len(), 4);

    let tos: Vec<_> = moves.into_iter().map(|m| m.to).collect();
    assert!(tos.contains(&pos(2, 2)));
    assert!(tos.contains(&pos(2, 4)));
    assert!(tos.contains(&pos(4, 2)));
    assert!(tos.contains(&pos(4, 4)));
}
