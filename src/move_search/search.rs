use super::super::position::*;

use rand::seq::SliceRandom;
use rand::thread_rng;

pub static mut VISITED_NODES: i32 = 0;

fn eval_piece(piece: Piece) -> f64 {
    match piece {
        Piece::Pawn => 1.0,
        Piece::Knight => 3.0,
        Piece::Bishop => 3.5,
        Piece::Rook => 5.0,
        Piece::Queen => 9.0,
        Piece::King => 0.0,
    }
}

fn eval_position(pos: &Position) -> f64 {
    let mut result = 0.0;
    for field in pos.board.iter() {
        match field {
            None => (),
            Some(PlayerPiece { player, piece }) => {
                let val = eval_piece(*piece);
                let sign = if pos.to_move == *player { 1.0 } else { -1.0 };
                result += val * sign;
            }
        }
    }
    result
}

fn negamax(pos: &Position, depth: i32) -> f64 {
    unsafe {
        VISITED_NODES += 1;
    }

    if pos.is_checkmate() {
        return -10000.0;
    } else if pos.is_stalemate() {
        return 0.0;
    }
    if depth == 0 {
        return eval_position(&pos);
    }

    let moves = pos.legal_moves();
    let mut best = -10000000.0;
    for &mv in moves.iter() {
        let mut pos2 = pos.clone();
        pos2.make_move(mv).unwrap();
        let val = -negamax(&pos2, depth - 1);
        if val > best {
            best = val;
        }
    }
    best
}

pub fn best_move_negamax(pos: &Position, depth: i32) -> Move {
    let moves = pos.legal_moves();
    let mut best = 0.0;
    let mut best_index = 0;
    for (index, mv) in moves.iter().enumerate() {
        unsafe {
            VISITED_NODES += 1;
        };
        let mut pos2 = pos.clone();
        pos2.make_move(*mv).unwrap();
        let val = -negamax(&pos2, depth - 1);
        if val > best || index == 0 {
            best = val;
            best_index = index;
        }
    }
    moves[best_index]
}

fn can_give_mate(pos: &Position) -> bool {
    for mv in pos.legal_moves() {
        let mut pos2 = pos.clone();
        pos2.make_move(mv).unwrap();
        if pos2.is_checkmate() {
            return true;
        }
    }
    return false;
}

pub fn choose_move(pos: &Position) -> Move {
    let mut rng = thread_rng();

    let mut good_moves = vec![];

    let moves = pos.legal_moves();
    for &mv in moves.iter() {
        let mut pos2 = pos.clone();
        pos2.make_move(mv).unwrap();
        if !can_give_mate(&pos2) {
            good_moves.push(mv);
        }
    }

    if good_moves.len() > 0 {
        return *good_moves.choose(&mut rng).unwrap();
    } else {
        return *moves.choose(&mut rng).unwrap();
    }
}

pub fn choose_move_rng(pos: &Position) -> Move {
    let mut rng = thread_rng();

    let moves = pos.legal_moves();
    return *moves.choose(&mut rng).unwrap();
    // return moves[0];
}
