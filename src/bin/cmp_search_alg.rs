use std::time;

use rust_chess::move_search::*;
use rust_chess::*;

#[derive(PartialEq)]
enum MoveSearcher {
    Negamax,
    AlphaBeta,
    AlphaBetaIter,
    PVS,
}

use MoveSearcher::*;

impl MoveSearcher {

    pub fn evaluation(self: &Self, pos: &mut Position, depth: i32) -> (IntMove, f64) {
        match self {
            Negamax =>
                negamax_top(pos, depth),

            AlphaBeta =>
                alphabeta_top(pos, depth),

            AlphaBetaIter =>
                alphabeta_iterative_deepening(pos, depth),

            PVS =>
                best_move_pvs(pos, depth),
        }
    }

    pub fn name(self: &Self) -> String {
        match self {
            Negamax =>
                format!("Negamax"),
            AlphaBeta =>
                format!("AlphaBeta"),
            AlphaBetaIter =>
                format!("AlphaBeta ItD"),
            PVS =>
               format!("PVS"),
        }
    }
}

fn main() {
    let positions = [
        // mate in 3
        "2rk2r1/3p2rr/8/1Q3Q2/6B1/8/3Q4/K7 w - - 0 1",
        // scholar's mate
        "r1bqkbnr/ppp2ppp/2np4/4p3/2B1P3/5Q2/PPPP1PPP/RNB1K1NR w KQkq - 0 4",
        // mate after a silent move
        "3r2k1/5p1p/5Bp1/8/5Q2/8/PPPP4/1K6 w - - 0 1",

        // starting
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        // random dynamic pos from chessbase blog
        "r1b2r2/pp4k1/1bpp1q1p/5ppQ/2B2NN1/2P5/P5PP/R1B1R2K b - - 0 22",
        // dynamic position, black in trouble
        "rnbqk1nr/3p1ppp/1p1P4/p3p3/PbB5/2N2Q2/1PPBNPPP/2KRR3 b kq - 0 1",
        // start of middle game
        "rnbqk1nr/3p1ppp/1pp1p3/p2P4/PbB1P3/2N2Q2/1PPBNPPP/2KRR3 b kq - 0 11",
        ];

    let searchers = [
        // Negamax,
        PVS,
        AlphaBeta,
        AlphaBetaIter,
    ];

    for (i, fen) in positions.iter().enumerate() {
        println!("#{} Comparing position {}", i+1, fen);

        for depth in 1..=5 {

            for searcher in searchers.iter() {
                if depth >= 5 && *searcher == Negamax {
                    continue;
                }

                let mut pos = Position::from_fen(fen);

                unsafe {
                    VISITED_NODES = 0;
                };

                let start = time::Instant::now();
                let (mv, ev) = searcher.evaluation(&mut pos, depth);
                let elapsed = start.elapsed();
                let move_ascii = intmove_to_uci_ascii(mv);
                let visited_nodes_safe = unsafe { VISITED_NODES };
                let nodes_per_sec = visited_nodes_safe as f64 / elapsed.as_secs_f64() / 1000.0;
                let elapsed_str = format!("{:.2?}", elapsed);
                println!(
                    "{:15} d={} | {} | {:.1} | {:10} | {:>8} | {:.0} knps",
                    searcher.name(), depth, move_ascii, ev, visited_nodes_safe, elapsed_str, nodes_per_sec
                );
            }
            println!();
        }
    }
}