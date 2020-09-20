use std::env;
use std::time;

use rust_chess::Position;

fn parse_args(args: Vec<String>) -> u32 {
    let default = 4;
    args.get(1)
        .map(|s| s.parse().ok())
        .flatten()
        .unwrap_or(default)
}

fn main() {
    let n_threads = parse_args(env::args().collect());
    println!(
        "Using {} thread{}.",
        n_threads,
        if n_threads > 1 { "s" } else { "" }
    );

    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    let expected: [u64;8] = [
        20,
        400,
        8_902,
        197_281,
        4_865_609,
        119_060_324,
        3_195_901_860,
        84_998_978_956,
    ];

    for (depth, &value) in expected.iter().enumerate() {
        let depth = depth as u32 + 1;
        let start = time::Instant::now();
        let result = Position::perft_mutable_par(depth, fen, n_threads);
        let elapsed = start.elapsed();
        let time_str = format!("{:.2?}", elapsed);
        let nps = result as f64 / elapsed.as_secs_f64() / 1000.0;
        println!(
            "perf imm {} = {:10} | correct: {} | total time{: >9} | {:5.0} knps | {:5.0} knps (thread)",
            depth,
            result,
            value == result,
            time_str,
            nps,
            nps / n_threads as f64
        );
    }
}
