use rust_chess::*;

// Test case 3. FEN '8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 1 1'
//   perf imm 1 =         14 | correct: true | total time 247.56µs |    57 knps |    14 knps (thread)
//   perf imm 2 =        191 | correct: true | total time 245.65µs |   778 knps |   194 knps (thread)
//   perf imm 3 =       2812 | correct: true | total time   1.43ms |  1965 knps |   491 knps (thread)
//   perf imm 4 =      43238 | correct: true | total time  12.51ms |  3455 knps |   864 knps (thread)
//   perf imm 5 =     674574 | correct: false | total time 176.89ms |  3814 knps |   953 knps (thread)
// Expected 674624 but got 674574. Diff = 50
// Test case result = FAILED

fn main() {
    let fen = "8/8/3p4/KPpr4/5p1k/8/4P1P1/4R3 w - - 2 3";
    let value = 15;
    let depth = 1;
    let mut total = 0u64;

    let mut pos = Position::from_fen(fen);

    for mv in pos.legal_moves() {
        let mut pos2 = pos.clone();
        pos2.make_move(mv);
        let result = if depth > 1 {
            Position::perft_mutable_par(depth-1, &pos2.to_fen(), 1)
        } else {
            1
        };
        total += result;
        println!("{}: {}", intmove_to_uci_ascii(mv), result);
    }
    println!("Total: {} ({})   [should be {}]", total, total == value, value);
}
