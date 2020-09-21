use std::time;

use rust_chess::move_search::*;
use rust_chess::position::*;

enum MoveSearcher {
    Negamax(i32),
    AlphaBeta(i32),
    PVS(i32),
}

impl MoveSearcher {
    pub fn bestmove(self: &Self, pos: &mut Position) -> IntMove {
        match self {
            MoveSearcher::Negamax(depth) =>
                best_move_negamax(pos, *depth),

            MoveSearcher::AlphaBeta(depth) =>
                best_move_alphabeta_negamax(pos, *depth),

            MoveSearcher::PVS(depth) =>
                best_move_pvs(pos, *depth).0,
        }
    }

    pub fn name(self: &Self) -> String {
        match self {
            MoveSearcher::Negamax(depth) =>
                format!("Negamax d = {}", depth),
            MoveSearcher::AlphaBeta(depth) =>
                format!("AlphaBeta d = {}", depth),
            MoveSearcher::PVS(depth) =>
               format!("PVS d = {}", depth),
        }
    }
}

fn main() {
    let positions = [
        // starting
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        // dynamic position, black in trouble
        "rnbqk1nr/3p1ppp/1p1P4/p3p3/PbB5/2N2Q2/1PPBNPPP/2KRR3 b kq - 0 1",
        // start of middle game
        "rnbqk1nr/3p1ppp/1pp1p3/p2P4/PbB1P3/2N2Q2/1PPBNPPP/2KRR3 b kq - 0 11",
    ];

    let searchers = [
        // MoveSearcher::Negamax(1),
        // MoveSearcher::Negamax(2),
        MoveSearcher::Negamax(3),
        // MoveSearcher::Negamax(4),
        // MoveSearcher::AlphaBeta(1),
        // MoveSearcher::AlphaBeta(2),
        // MoveSearcher::AlphaBeta(3),
        MoveSearcher::AlphaBeta(4),
        MoveSearcher::AlphaBeta(5),
        // MoveSearcher::PVS(1),
        // MoveSearcher::PVS(2),
        // MoveSearcher::PVS(3),
        MoveSearcher::PVS(4),
        MoveSearcher::PVS(5),
    ];

    for fen in positions.iter() {
        println!("\nBenchmarking position {}", fen);

        for searcher in searchers.iter() {
            let mut pos = Position::from_fen(fen);

            unsafe {
                VISITED_NODES = 0;
            };

            let start = time::Instant::now();
            let mv = searcher.bestmove(&mut pos);
            let elapsed = start.elapsed();
            let move_ascii = intmove_to_uci_ascii(mv);
            let visited_nodes_safe = unsafe { VISITED_NODES };
            let nodes_per_sec = visited_nodes_safe as f64 / elapsed.as_secs_f64() / 1000.0;
            let elapsed_str = format!("{:.2?}", elapsed);
            println!(
                "{:20} | {} | {:10} | {:>8} | {:.0} knps",
                searcher.name(), move_ascii, visited_nodes_safe, elapsed_str, nodes_per_sec
            );
        }
    }
}
