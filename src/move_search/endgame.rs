use std::collections::HashMap;

use super::super::position::*;

pub type RESULT = i32;
const WIN_W: RESULT = 1;
const DRAW: RESULT = 0;
const WIN_B: RESULT = -1;

pub type TABLE = HashMap<u64, (RESULT, u64, IntMove)>;

pub fn generate_index_pairs() -> Vec<(usize, usize)> {
    let mut v = vec![];
    for &k1 in INDEXES88.iter() {
        for &k2 in INDEXES88.iter() {
            if k1 != k2 {
                v.push((k1, k2));
            }
        }
    }
    v
}


pub fn generate_index_triples() -> Vec<(usize, usize, usize)> {
    let mut v = vec![];
    for &k1 in INDEXES88.iter() {
        for &k2 in INDEXES88.iter() {
            if k1 != k2 {
                for &k3 in INDEXES88.iter() {
                    if k1 != k3 && k2 != k3 {
                        v.push((k1, k2, k3));
                    }
                }
            }
        }
    }
    v
}

pub fn generate_kings_only(table: &mut TABLE) {
    for &(k1, k2) in generate_index_pairs().iter() {
        for &pl in PLAYERS.iter() {
            let mut board = vec![0; FIELDS88];
            let castle_rights = [[false; 2]; 2];

            board[k1] = W_KING;
            board[k2] = B_KING;

            let pos = Position::create(board, pl, None, castle_rights, 0, 0, [k1, k2]);
            if !pos.is_king_in_check(pl.opposite()) {
                table.insert(pos.hash, (DRAW, 0, 0));
            }
        }
    }
    println!("table size after k vs k generation = {}", table.len());
}

pub fn make_position_kq_vs_q(k1: usize, k2: usize, q: usize, pl: Player) -> Option<Position> {
    let mut board = vec![0; FIELDS88];
    let castle_rights = [[false; 2]; 2];

    board[k1] = W_KING;
    board[k2] = B_KING;
    board[q] = W_QUEEN;

    let pos = Position::create(board, pl, None, castle_rights, 0, 0, [k1, k2]);
    if !pos.is_king_in_check(pl.opposite()) {
        Some(pos)
    } else {
        None
    }
}

pub fn analyze(table: &mut TABLE) {
    let mut analyzed = 0;
    let mut max_count = 0;

    let mut iteration = 0;
    let mut changed = true;

    let mut checkmates = 0;
    let mut stats = vec![0; 3];

    let mut done = vec![];

    while changed {
        changed = false;
        let mut current = 0;

        for &k1 in INDEXES88.iter() {
            for &k2 in INDEXES88.iter() {
                if k1 != k2 {
                    for &q in INDEXES88.iter() {
                        if k1 != q && k2 != q {
                            for &pl in PLAYERS.iter() {
                                let spos = make_position_kq_vs_q(k1, k2, q, pl);
                                if let Some(mut pos) = spos {
                                    if iteration == 0 {
                                        done.push(false);
                                    }

                                    if !done[current] && !table.contains_key(&pos.hash) {
                                        let my_win = if pos.to_move == Player::White { WIN_W } else { WIN_B };
                                        let moves = pos.legal_moves();

                                        if moves.len() == 0 {
                                            if pos.is_king_in_check(pos.to_move) {
                                                checkmates += 1;
                                                table.insert(pos.hash, (-my_win, 0, 0));
                                                stats[(1 -my_win) as usize] += 1;
                                            } else {
                                                table.insert(pos.hash, (DRAW, 0, 0));
                                                stats[1+DRAW as usize] += 1;
                                            }
                                            done[current] = true;
                                            changed = true;
                                            analyzed += 1;
                                        } else {
                                            let mut has_win = false;
                                            let mut all_in_hash = true;
                                            let mut results = vec![vec![]; 3];

                                            for mv in moves.iter() {
                                                pos.make_move(*mv).unwrap();

                                                match table.get(&pos.hash) {
                                                    None => {
                                                        all_in_hash = false;
                                                    },
                                                    Some((result, count, _mv)) => {
                                                        if *result == my_win {
                                                            has_win = true;
                                                        }
                                                        results[(*result+1) as usize].push((*count, *mv));
                                                    }
                                                }
                                                pos.unmake_move(*mv).unwrap();
                                            }
                                            if has_win || all_in_hash {
                                                let best;

                                                let (move_count, best_move) = if has_win {
                                                    best = my_win;
                                                    results[(my_win + 1) as usize].iter().min_by_key(|(c,_m)| c).unwrap()
                                                } else if results[(DRAW + 1) as usize].len() > 0 {
                                                    best = DRAW;
                                                    results[(DRAW + 1) as usize].iter().max_by_key(|(c,_m)| c).unwrap()
                                                } else {
                                                    best = -my_win;
                                                    results[(-my_win + 1) as usize].iter().max_by_key(|(c,_m)| c).unwrap()
                                                };

                                                // CHECK IF WE REALLY NEED THIS CHECK
                                                if (1 + *move_count) <= iteration+1 {
                                                    table.insert(pos.hash, (best, 1 + *move_count, *best_move));
                                                    stats[1 + best as usize] += 1;

                                                    max_count = max_count.max(1 + *move_count);
                                                    analyzed += 1;
                                                    changed = true;
                                                    done[current] = true;
                                                }
                                            }
                                        }
                                    }

                                    current += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        iteration += 1;
        println!("total = {}, analyzed = {} max_count = {} checkmates = {}", done.len(), analyzed, max_count, checkmates);
    }
    println!("Final stats:");
    println!("  White wins: {:6}", stats[(1 + WIN_W) as usize]);
    println!("  Draw      : {:6}", stats[(1 + DRAW) as usize]);
    println!("  Black wins: {:6}", stats[(1 + WIN_B) as usize]);
}

pub fn initialize_table() -> TABLE {
    HashMap::new()
}

pub fn generate_endgame_table(table: &mut TABLE) {
    // let mut table = HashMap::new();
    generate_kings_only(table);
    analyze(table);
}
