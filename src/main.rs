use rust_chess::Position;

fn main() {
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

    for (i, &value) in expected.iter().enumerate() {
        let i = i as u32;
        let result = Position::perft_immutable_par(i+1, fen);
        println!("perf imm {} = {:#?} | correct: {}", i+1, result, value == result);
    }
}