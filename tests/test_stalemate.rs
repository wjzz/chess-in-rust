use rust_chess::*;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn stalemated_king() {
        let fens = [
            // K+P vs K
            "4k3/4P3/4K3/8/8/8/8/8 b - - 0 1",
            // K+Q vs K
            "5k2/8/4Q1K1/8/8/8/8/8 b - - 0 1",
        ];
        for fen in fens.iter() {
            let pos = Position::from_fen(fen);
            assert!(pos.is_stalemate());
        }
    }

    #[test]
    fn non_stalemated_king() {
        let fens = [
            // K+P vs K
            "4k3/4P3/4K3/8/8/8/8/8 w - - 0 1",
            // K+Q vs K
            "5k2/8/4Q1K1/8/8/8/8/8 w - - 0 1",
        ];
        for fen in fens.iter() {
            let pos = Position::from_fen(fen);
            assert!(!pos.is_stalemate());
        }
    }

    // #[test]
    // fn not_mated_king() {
    //     let fens = [
    //         // initial pos
    //         "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
    //         // single queen check - can take
    //         "r1bqkbnr/pppp1Qpp/2n5/4p3/4P3/8/PPPP1PPP/RNB1KBNR b KQkq - 0 3",
    //         // single queen check - can block
    //         "rnbqkbnr/pppp2pp/5p2/4p2Q/4P3/8/PPPP1PPP/RNB1KBNR b KQkq - 1 3",
    //         // fool's mate but king can escape
    //         "r1b1kbnr/pppp1ppp/2n5/4p3/6Pq/3P1P2/PPP1P2P/RNBQKBNR w KQkq - 1 4",
    //     ];
    //     for fen in fens.iter() {
    //         let pos = Position::from_fen(fen);
    //         assert!(!pos.is_checkmate());
    //     }
    // }
}
