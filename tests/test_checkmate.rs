use rust_chess::*;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn mated_king() {
        let fens = [
            // fool's mate
            "rnb1kbnr/pppp1ppp/8/4p3/6Pq/5P2/PPPPP2P/RNBQKBNR w KQkq - 1 3",
            // scholar's mate
            "r1bqkb1r/pppp1Qpp/2n2n2/4p3/2B1P3/8/PPPP1PPP/RNB1K1NR b KQkq - 0 4",
            // dutch defense blunder
            "rnbqkbnr/ppppp2p/8/5p1Q/3P4/8/PPP2PPP/RNB1KBNR b KQkq - 1 4",
            // caro-kann smothered king
            "r1bqkb1r/pp1npppp/2pN1n2/8/3P4/8/PPP1QPPP/R1B1KBNR b KQkq - 4 6",
        ];
        for fen in fens.iter() {
            let pos = Position::from_fen(fen);
            println!("{}", fen);
            println!("legal moves len = {}", pos.legal_moves().len());
            println!("{:?}", pos.legal_moves());
            assert!(pos.is_checkmate());
        }
    }

    #[test]
    fn not_mated_king() {
        let fens = [
            // initial pos
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            // single queen check - can take
            "r1bqkbnr/pppp1Qpp/2n5/4p3/4P3/8/PPPP1PPP/RNB1KBNR b KQkq - 0 3",
            // single queen check - can block
            "rnbqkbnr/pppp2pp/5p2/4p2Q/4P3/8/PPPP1PPP/RNB1KBNR b KQkq - 1 3",
            // fool's mate but king can escape
            "r1b1kbnr/pppp1ppp/2n5/4p3/6Pq/3P1P2/PPP1P2P/RNBQKBNR w KQkq - 1 4",
        ];
        for fen in fens.iter() {
            let pos = Position::from_fen(fen);
            assert!(!pos.is_checkmate());
        }
    }
}
