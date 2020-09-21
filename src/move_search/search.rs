use super::super::position::*;

use std::time;

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
    for &field in pos.board.iter() {
        if field != EMPTY {
            let (player, piece) = boardcell_destruct(field);
            let val = eval_piece(piece);
            let sign = if pos.to_move == player { 1.0 } else { -1.0 };
            result += val * sign;
        }
    }
    result
}

fn negamax(pos: &mut Position, depth: i32) -> f64 {
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
        // let mut pos2 = pos.clone();
        // pos2.make_move(mv).unwrap();
        // let val = -negamax(&pos2, depth - 1);
        pos.make_move(mv).unwrap();
        let val = -negamax(&mut *pos, depth - 1);
        pos.unmake_move(mv).unwrap();

        if val > best {
            best = val;
        }
    }
    best
}

pub fn best_move_negamax(pos: &mut Position, depth: i32) -> IntMove {
    let moves = pos.legal_moves();
    let mut best = 0.0;
    let mut best_index = 0;
    for (index, mv) in moves.iter().enumerate() {
        unsafe {
            VISITED_NODES += 1;
        }
        pos.make_move(*mv).unwrap();
        let val = -negamax(&mut *pos, depth - 1);
        pos.unmake_move(*mv).unwrap();

        if val > best || index == 0 {
            best = val;
            best_index = index;
        }
    }
    moves[best_index]
}

// function negamax(node, depth, α, β, color) is
//     if depth = 0 or node is a terminal node then
//         return color × the heuristic value of node

//     childNodes := generateMoves(node)
//     childNodes := orderMoves(childNodes)
//     value := −∞
//     foreach child in childNodes do
//         value := max(value, −negamax(child, depth − 1, −β, −α, −color))
//         α := max(α, value)
//         if α ≥ β then
//             break (* cut-off *)
//     return value

// (* Initial call for Player A's root node *)
// negamax(rootNode, depth, −∞, +∞, 1)

static mut ALPHA_BETA_BEST_MOVE: Option<IntMove> = None;
static mut RESULT_VEC: Vec<f64> = vec![];
static mut ORDERING: Vec<IntMove> = vec![];

// static mut PV: Vec<IntMove> = vec![];

fn alphabeta_negamax(pos: &mut Position, level: i32, depth: i32, alpha: f64, beta: f64) -> (f64, Option<String>) {
    unsafe {
        VISITED_NODES += 1;
    }

    if pos.is_checkmate() {
        return (-10000.0, None);
    } else if pos.is_stalemate() {
        return (0.0, None);
    }
    if depth == 0 {
        return (eval_position(&pos), None);
    }

    let mut alpha = alpha;

    let moves = pos.legal_moves();
    let mut best = -10000000.0;
    let mut pv = String::new();

    for &mv in moves.iter() {
        pos.make_move(mv).unwrap();
        let (val, best_reply) = alphabeta_negamax(&mut *pos, level+1, depth - 1, -beta, -alpha);
        let val = -val;
        pos.unmake_move(mv).unwrap();

        if val > best {
            best = val;
            pv = intmove_to_uci_ascii(mv) + " " + &best_reply.unwrap_or(String::from(""));
            if level == 0 {
                unsafe {
                    ALPHA_BETA_BEST_MOVE = Some(mv);
                }
            }
        }

        alpha = alpha.max(best);
        if alpha >= beta {
            break;
        }
    }
    (best, Some(pv))
}

pub fn best_move_iterative_deepening(pos: &mut Position, depth: i32) -> IntMove {

    for d in 1..=depth {
        let start = std::time::Instant::now();
        unsafe {
            VISITED_NODES = 0;
        }

        let (val, pv) = alphabeta_negamax(&mut *pos, 0, d, -1_000_000.0, 1_000_000.0);

        let cp = (100.0 * val) as i32;
        let nodes = unsafe { VISITED_NODES };
        // let pv: Vec<String> = unsafe { PV.iter().map(|mv| intmove_to_uci_ascii(*mv)).collect() };
        // let pv = pv.join(" ");

        let pv = pv.unwrap_or(String::from(""));
        println!("info score cp {} depth {} nodes {} time {} pv {}", cp, d, nodes, start.elapsed().as_millis(), pv);
        // 		info score cp 13  depth 1 nodes 13 time 15 pv f1b5
    }
    let best_move = unsafe {
        ALPHA_BETA_BEST_MOVE.unwrap()
    };
    return best_move;
}

// function pvs(node, depth, α, β, color) is
//     if depth = 0 or node is a terminal node then
//         return color × the heuristic value of node
//     for each child of node do
//         if child is first child then
//             score := −pvs(child, depth − 1, −β, −α, −color)
//         else
//             score := −pvs(child, depth − 1, −α − 1, −α, −color) (* search with a null window *)
//             if α < score < β then
//                 score := −pvs(child, depth − 1, −β, −score, −color) (* if it failed high, do a full re-search *)
//         α := max(α, score)
//         if α ≥ β then
//             break (* beta cut-off *)
//     return α


fn pvs(pos: &mut Position, level: i32, depth: i32, alpha: f64, beta: f64) -> f64 {
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

    let mut alpha = alpha;

    let moves = pos.legal_moves();

    // TODO: add ORDERING
    let mut first_node = true;
    for &mv in moves.iter() {
        pos.make_move(mv).unwrap();

        let val = if first_node {
            -pvs(&mut *pos, level+1, depth - 1, -beta, -alpha)
        } else {
            let val = -pvs(&mut *pos, level+1, depth - 1, -alpha-1.0, -alpha);
            if alpha < val && val < beta {
                -pvs(&mut *pos, level+1, depth - 1, -beta, -val)
            } else {
                val
            }
        };
        pos.unmake_move(mv).unwrap();
        first_node = false;

        if val > alpha {
            if level == 0 {
                unsafe {
                    ALPHA_BETA_BEST_MOVE = Some(mv);
                }
            }
        }

        alpha = alpha.max(val);
        if alpha >= beta {
            break;
        }
    }
    alpha
}


pub fn best_move_pvs(pos: &mut Position, depth: i32) -> (IntMove, f64) {
    let val = pvs(&mut *pos, 0, depth, -1_000_000.0, 1_000_000.0);
    let best_move = unsafe {
        ALPHA_BETA_BEST_MOVE.unwrap()
    };
    return (best_move, val);
}



fn can_give_mate(pos: &mut Position) -> bool {
    for mv in pos.legal_moves() {
        let mut pos2 = pos.clone();
        pos2.make_move(mv).unwrap();
        if pos2.is_checkmate() {
            return true;
        }
    }
    return false;
}

pub fn choose_move(pos: &mut Position) -> IntMove {
    let mut rng = thread_rng();

    let mut good_moves = vec![];

    let moves = pos.legal_moves();
    for &mv in moves.iter() {
        let mut pos2 = pos.clone();
        pos2.make_move(mv).unwrap();
        if !can_give_mate(&mut pos2) {
            good_moves.push(mv);
        }
    }

    if good_moves.len() > 0 {
        return *good_moves.choose(&mut rng).unwrap();
    } else {
        return *moves.choose(&mut rng).unwrap();
    }
}

pub fn choose_move_rng(pos: &mut Position) -> IntMove {
    let mut rng = thread_rng();

    let moves = pos.legal_moves();
    return *moves.choose(&mut rng).unwrap();
    // return moves[0];
}
