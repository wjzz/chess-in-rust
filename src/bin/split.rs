use rust_chess::*;

// 16,17c16,17
// < d5d6: 79551
// < d5e6: 97464
// ---
// > d5d6: 79604
// > d5e6: 97470


// r3k2r/p1ppqpb1/bn1Ppnp1/4N3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R b KQkq - 0 1
// r3k2r/p1ppqpb1/bn2Pnp1/4N3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R b KQkq - 0 1

fn main() {
    // let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 1 1";
    let fen = "r3k3/p1ppPpb1/bn2pnp1/4N2r/1p2P3/2N2Q1p/PPPBBPPP/R3K2R b KQq - 0 2";
    let value = 37;
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
