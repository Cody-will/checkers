
// tests/forced_capture.rs
mod common;
use common::*;

use checkers::game::rules::{get_moves, ForcedMoveState};
use checkers::game::types::{ Square, PlayerSide};

#[test]
fn when_any_capture_exists_get_moves_returns_only_jumps_and_mustcapture_state() {
    let mut b = empty_board();

    // Red man that can jump:
    // Red at (4,3), Black at (3,2), landing (2,1) empty
    let from = pos(4, 3);
    set_sq(&mut b, from, Square::RedBase);
    set_sq(&mut b, pos(3, 2), Square::BlackBase);

    // Also create another red piece that *could* slide, to ensure slides are suppressed.
    set_sq(&mut b, pos(6, 1), Square::RedBase);

    let (moves, forced) = get_moves(&b, &PlayerSide::Red);

    // Should be only jump moves, not slides
    assert!(!moves.is_empty());
    for m in &moves {
        // Jumps always move 2 rows in your logic
        assert_eq!((m.to.row - m.from.row).abs(), 2, "expected only jumps when forced capture exists");
    }

    match forced {
        ForcedMoveState::MustCapture { from: sources } => {
            assert!(sources.contains(&from), "expected MustCapture sources to include the jumping piece");
        }
        _ => panic!("expected ForcedMoveState::MustCapture"),
    }
}

#[test]
fn mustcapture_sources_lists_all_pieces_that_have_a_jump() {
    let mut b = empty_board();

    // Two red pieces, both can jump
    let a = pos(4, 3);
    let bpos = pos(4, 5);

    set_sq(&mut b, a, Square::RedBase);
    set_sq(&mut b, bpos, Square::RedBase);

    set_sq(&mut b, pos(3, 2), Square::BlackBase); // for a -> (2,1)
    set_sq(&mut b, pos(3, 6), Square::BlackBase); // for bpos -> (2,7)

    let (_moves, forced) = get_moves(&b, &PlayerSide::Red);

    match forced {
        ForcedMoveState::MustCapture { from: sources } => {
            assert!(sources.contains(&a));
            assert!(sources.contains(&bpos));
            assert_eq!(sources.len(), 2);
        }
        _ => panic!("expected MustCapture"),
    }
}

