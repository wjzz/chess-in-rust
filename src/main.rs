use std::env;
use std::time;

use rust_chess::Position;

fn parse_args(args: Vec<String>) -> u32 {
    let default = 4;
    args.get(1).map(|s| s.parse().ok()).flatten().unwrap_or(default)
}

fn main() {
    let n_threads = parse_args(env::args().collect());
    println!(
        "Using {} thread{}.",
        n_threads,
        if n_threads > 1 { "s" } else { "" }
    );

    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    let expected = [
        20,
        400,
        8_902,
        197_281,
        4_865_609,
        119_060_324,
        3_195_901_860,
    ];

    for (depth, &value) in expected.iter().enumerate() {
        let depth = depth as u32 + 1;
        let start = time::Instant::now();
        let result = Position::perft_immutable_par(depth, fen, n_threads);
        let time_str = format!("{:.2?}", start.elapsed());
        println!(
            "perf imm {} = {:10} | correct: {} | total time{: >9}",
            depth,
            result,
            value == result,
            time_str
        );
    }
}
