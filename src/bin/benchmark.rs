use std::time;

use rust_chess::move_search::*;
use rust_chess::position::*;

fn main() {
    let positions = [
        // starting
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        // dynamic position, black in trouble
        "rnbqk1nr/3p1ppp/1p1P4/p3p3/PbB5/2N2Q2/1PPBNPPP/2KRR3 b kq - 0 1",
        // start of middle game
        "rnbqk1nr/3p1ppp/1pp1p3/p2P4/PbB1P3/2N2Q2/1PPBNPPP/2KRR3 b kq - 0 11",
    ];

    for fen in positions.iter() {
        println!("Benchmarking position {}", fen);

        for depth in 1..=4 {
            let mut pos = Position::from_fen(fen);

            unsafe {
                visited_nodes = 0;
            };

            let start = time::Instant::now();
            // let mv = best_move_negamax(&mut pos, depth);
            // let mv = best_move_alphabeta_negamax(&mut pos, depth);
            let mv = best_move_pvs(&mut pos, depth);
            let elapsed = start.elapsed();
            let move_ascii = mv.to_usi_ascii();
            let visited_nodes_safe = unsafe { visited_nodes };
            let nodes_per_sec = visited_nodes_safe as f64 / elapsed.as_secs_f64() / 1000.0;
            println!(
                "Negamax d={} | {} | {:10} | {:.2?} | {:.0} knps",
                depth, move_ascii, visited_nodes_safe, elapsed, nodes_per_sec
            );
        }
    }
}
