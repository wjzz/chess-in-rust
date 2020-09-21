use std::env;
use std::time;
use ansi_term::Colour;

use rust_chess::Position;

fn parse_args(args: Vec<String>) -> u32 {
    let default = 4;
    args.get(1)
        .map(|s| s.parse().ok())
        .flatten()
        .unwrap_or(default)
}

struct TestCase {
    fen: &'static str,
    expected: Vec<u64>,
    depth: usize,
}

fn get_tests() -> Vec<TestCase> {
    vec![
        // starting position
        TestCase {
            fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            expected: vec![
                20,
                400,
                8_902,
                197_281,
                4_865_609,
                119_060_324,
                3_195_901_860,
                84_998_978_956,
            ],
            depth: 5
        },
        // kiwipete
        // https://www.chessprogramming.org/Perft_Results#Position_2
        TestCase {
            fen: "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 1 1",
            expected: vec![
                48,
                2_039,
                13_310,
                97_862,
                4_085_603,
                193_690_690,
                8_031_647_685
            ],
            depth: 5
        },
        // http://cinnamonchess.altervista.org/perft.html
        // second to last, lonely white king
        TestCase {
            fen: "8/3K4/2p5/p2b2r1/5k2/8/8/1q6 b - - 1 67",
            expected: vec![
                50,
                279,
                13_310,
                54_703,
                2_538_084,
                10_809_689,
            ],
            depth: 5
        },
    ]
}

fn main() {
    let n_threads = parse_args(env::args().collect());
    println!(
        "Using {} thread{}.",
        n_threads,
        if n_threads > 1 { "s" } else { "" }
    );

    let tests = get_tests();
    let mut all_good = true;
    for TestCase { fen, expected, depth: max_depth } in tests.iter() {
        let mut ok = true;
        for depth in 1..=*max_depth {
            let value = expected[depth-1];
            let start = time::Instant::now();
            let result = Position::perft_mutable_par(depth as u32, fen, n_threads);
            let elapsed = start.elapsed();
            let time_str = format!("{:.2?}", elapsed);
            let nps = result as f64 / elapsed.as_secs_f64() / 1000.0;
            println!(
                "  perf imm {} = {:10} | correct: {} | total time{: >9} | {:5.0} knps | {:5.0} knps (thread)",
                depth,
                result,
                value == result,
                time_str,
                nps,
                nps / n_threads as f64
            );
            if value != result {
                ok = false;
                all_good = false;
                println!("Expected {} but got {}", value, result);
                break;
            }
        }
        let result_str = if ok { Colour::Green.paint("SUCCEED") } else { Colour::Red.paint("FAILED") };
        println!("Test case result = {}", result_str);
    }
    let result_str = if all_good { Colour::Green.bold().paint("TEST SUITE SUCCEED") } else { Colour::Red.bold().paint("TEST SUITE FAILED") };
    println!("{}", result_str);
}
