use rand::seq::SliceRandom;
use rand::thread_rng;

use super::super::position::*;

pub static mut VISITED_NODES: u64 = 0;

pub const CHECKMATE_EV: f64 = 10000.0;

const MATERIAL_VAL: [f64; 13] = [
     0.0 , // B_KING
    -9.0, // B_QUEEN
    -5.0, // B_ROOK
    -3.5, // B_BISHOP
    -3.0, // B_KNIGHT
    -1.0, // B_PAWN
    0.0, // EMPTY
    1.0, // W_PAWN
    3.0, // W_KNIGHT
    3.5, // W_BISHOP
    5.0, // W_ROOK
    9.0, // W_QUEEN
    0.0, // W_KING
];

fn eval_material(pos: &Position) -> f64 {
    let mut ev = 0.0;
    for i in INDEXES88.iter() {
        ev += MATERIAL_VAL[(pos.board[*i] + 6) as usize];
    }
    if pos.to_move == Player::White { ev } else { -ev }
}

fn eval_position(pos: &Position) -> f64 {
    return eval_material(&pos);
}

static mut BEST_MOVE: Option<IntMove> = None;

fn negamax(pos: &mut Position, level: i32, depth: i32) -> f64 {
    unsafe {
        VISITED_NODES += 1;
    }

    let moves = pos.legal_moves();

    if moves.len() == 0 {
        if pos.is_king_in_check(pos.to_move) {
            return -CHECKMATE_EV;
        } else {
            return 0.0;
        }
    }

    if depth == 0 {
        return eval_position(&pos);
    }

    let mut best = -10000000.0;
    for &mv in moves.iter() {
        pos.make_move(mv).unwrap();
        let val = -negamax(&mut *pos, level+1, depth - 1);
        pos.unmake_move(mv).unwrap();

        if val > best {
            best = val;

            if level == 0 {
                unsafe {
                    BEST_MOVE = Some(mv);
                }
            }

        }
    }
    best
}

pub fn negamax_top(pos: &mut Position, depth: i32) -> (IntMove, f64) {
    unsafe { BEST_MOVE = None; }

    let val = negamax(&mut *pos, 0, depth);

    let best_move = unsafe {
        BEST_MOVE.unwrap()
    };
    return (best_move, val);
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

// static mut RESULT_VEC: Vec<f64> = vec![];
// static mut ORDERING: Vec<IntMove> = vec![];

// static mut PV: Vec<IntMove> = vec![];

fn alphabeta(pos: &mut Position, level: i32, depth: i32, alpha: f64, beta: f64) -> (f64, Option<String>) {
    unsafe {
        VISITED_NODES += 1;
    }

    let moves = pos.legal_moves();

    if moves.len() == 0 {
        if pos.is_king_in_check(pos.to_move) {
            return (-CHECKMATE_EV, None);
        } else {
            return (0.0, None);
        }
    }

    if depth == 0 {
        return (eval_position(&pos), None);
    }

    let mut alpha = alpha;

    let mut best = -10000000.0;
    let mut pv = String::new();

    for &mv in moves.iter() {
        pos.make_move(mv).unwrap();
        let (val, best_reply) = alphabeta(&mut *pos, level+1, depth - 1, -beta, -alpha);
        let val = -val;
        pos.unmake_move(mv).unwrap();

        if val > best {
            best = val;
            pv = intmove_to_uci_ascii(mv) + " " + &best_reply.unwrap_or(String::from(""));
            if level == 0 {
                unsafe {
                    BEST_MOVE = Some(mv);
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

pub fn alphabeta_top(pos: &mut Position, depth: i32) -> (IntMove, f64) {
    unsafe { BEST_MOVE = None; }

    let (val, _pv) = alphabeta(&mut *pos, 0, depth, -CHECKMATE_EV, CHECKMATE_EV);

    let best_move = unsafe {
        BEST_MOVE.unwrap()
    };
    return (best_move, val);
}

pub fn alphabeta_iterative_deepening(pos: &mut Position, depth: i32) -> (IntMove, f64) {
    let mut best_val = 0.0;
    let mut total_nodes = 0u64;

    for d in 1..=depth {
        let start = std::time::Instant::now();
        unsafe {
            // TODO: we may want to use this info later for move ordering
            BEST_MOVE = None;
            VISITED_NODES = 0;
        }

        let (val, pv) = alphabeta(&mut *pos, 0, d, -CHECKMATE_EV, CHECKMATE_EV);
        best_val = val;

        let cp = (100.0 * val) as i32;
        let nodes = unsafe { VISITED_NODES };
        total_nodes += nodes;
        // let pv: Vec<String> = unsafe { PV.iter().map(|mv| intmove_to_uci_ascii(*mv)).collect() };
        // let pv = pv.join(" ");

        let pv = pv.unwrap_or(String::from(""));
        // println!("info score cp {} depth {} nodes {} time {} pv {}", cp, d, nodes, start.elapsed().as_millis(), pv);

        // 		info score cp 13  depth 1 nodes 13 time 15 pv f1b5

        if val == CHECKMATE_EV {
            break;
        }
    }
    unsafe { VISITED_NODES = total_nodes; }
    let best_move = unsafe {
        BEST_MOVE.unwrap()
    };
    return (best_move, best_val);
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

    let moves = pos.legal_moves();

    if moves.len() == 0 {
        if pos.is_king_in_check(pos.to_move) {
            return -CHECKMATE_EV;
        } else {
            return 0.0;
        }
    }

    if depth == 0 {
        return eval_position(&pos);
    }

    let mut alpha = alpha;

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
                    BEST_MOVE = Some(mv);
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
        BEST_MOVE.unwrap()
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
