use crate::game::types::{Position, PlayerSide, Direction, PlayerMove};
use crate::game::board::Board;

#[derive(Debug, Clone)]
pub enum ForcedMoveState {
    None,
    MustCapture { from: Vec<Position> },
    ContinuingCombo { from: Position },
}

fn in_bounds(p: &Position) -> bool {
    (0..=7).contains(&p.row) && (0..=7).contains(&p.col)
}

pub fn get_moves(board: &Board, side: &PlayerSide) -> (Vec<PlayerMove>, ForcedMoveState) {
    let mut slides_all: Vec<PlayerMove> = Vec::new();
    let mut jumps_all: Vec<PlayerMove> = Vec::new();
    let mut jumps_from: Vec<Position> = Vec::new();

    for row in 0..8 {
        for col in 0..8 {
            let selected = Position::new(row, col);
            if !board.get_value(&selected).is_players(side) {
                continue;
            }

            let slides = get_slides(&selected, board, side);
            let jumps = get_jumps(&selected, board, side);

            slides_all.extend(slides);

            if !jumps.is_empty() {
                jumps_from.push(selected);
                jumps_all.extend(jumps);
            }
        }
    }

    if !jumps_all.is_empty() {
        (jumps_all, ForcedMoveState::MustCapture { from: jumps_from })
    } else {
        (slides_all, ForcedMoveState::None)
    }
}

fn get_slides(from: &Position, board: &Board, side: &PlayerSide) -> Vec<PlayerMove> {
    let mut moves = Vec::new();
    let dir = Direction::current(side);

        let deltas: &[(i8, i8)] = match dir {
        Direction::Up => &[(-1, -1), (-1, 1)],
        Direction::Down => &[(1, -1), (1, 1)],
    };

    for (d_row, d_col) in deltas {
        let to = Position::new(from.row + d_row, from.col + d_col);
        if !in_bounds(&to) { continue; }
        if board.get_value(&to).is_empty() {
            moves.push(PlayerMove::new(from.clone(), to));
        }
    }

    moves
}

pub fn get_jumps(from: &Position, board: &Board, side: &PlayerSide) -> Vec<PlayerMove> {
    let mut jumps = Vec::new();
    let dir = Direction::current(side);

    let deltas: &[(i8, i8)] = match dir {
        Direction::Up => &[(-1, -1), (-1, 1)],
        Direction::Down => &[(1, -1), (1, 1)],
    };

    for (d_row, d_col) in deltas {
        let mid = Position::new(from.row + d_row, from.col + d_col);
        let to  = Position::new(from.row + 2*d_row, from.col + 2*d_col);

        if !in_bounds(&mid) || !in_bounds(&to) { continue; }

        if board.get_value(&to).is_empty()
            && !board.get_value(&mid).is_empty()
            && !board.get_value(&mid).is_players(side)
        {
            jumps.push(PlayerMove::new(from.clone(), to));
        }
    }

    jumps
}




