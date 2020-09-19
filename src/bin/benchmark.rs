use std::time;

use rust_chess::move_search::*;
use rust_chess::position::*;

enum MoveSearcher {
    Negamax(i32),
}

impl MoveSearcher {
    pub fn bestmove(self: &Self, pos: &Position) -> Move {
        match self {
            MoveSearcher::Negamax(depth) =>
                best_move_negamax(pos, *depth)
        }
    }

    pub fn name(self: &Self) -> String {
        match self {
            MoveSearcher::Negamax(depth) =>
                format!("Negamax d = {}", depth)
        }
    }
}

fn main() {
    let positions = [
        // start of middle game
        "rnbqk1nr/3p1ppp/1pp1p3/p2P4/PbB1P3/2N2Q2/1PPBNPPP/2KRR3 b kq - 0 11",
        // starting
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        // dynamic position, black in trouble
        "rnbqk1nr/3p1ppp/1p1P4/p3p3/PbB5/2N2Q2/1PPBNPPP/2KRR3 b kq - 0 1",
    ];

    let searchers = [
        MoveSearcher::Negamax(1),
        MoveSearcher::Negamax(2),
        // MoveSearcher::Negamax(3),
    ];

    for fen in positions.iter() {
        println!("Benchmarking position {}", fen);

        for searcher in searchers.iter() {
            let pos = Position::from_fen(fen);

            unsafe {
                VISITED_NODES = 0;
            };

            let start = time::Instant::now();
            let mv = searcher.bestmove(&pos);
            let elapsed = start.elapsed();
            let move_ascii = mv.to_usi_ascii();
            let visited_nodes_safe = unsafe { VISITED_NODES };
            let nodes_per_sec = visited_nodes_safe as f64 / elapsed.as_secs_f64() / 1000.0;
            println!(
                "{} | {} | {:10} | {:.2?} | {:.0} knps",
                searcher.name(), move_ascii, visited_nodes_safe, elapsed, nodes_per_sec
            );
        }
    }
}
