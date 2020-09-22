use rust_chess::*;

#[cfg(test)]
mod test_moves {

    use super::*;

    #[test]
    fn test_eval_startpos() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

        let mut pos = Position::from_fen(fen);
        let moves = pos.legal_moves();

        for &mv in moves.iter() {
            pos.make_move(mv).unwrap();
            let ev = pos.eval();
            pos.unmake_move(mv).unwrap();
            println!("{} => {:+.1}", intmove_to_uci_ascii(mv), ev);
        }

        assert_eq!(20, moves.len());
    }

    #[test]
    fn test_hash_move() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

        let mut pos = Position::from_fen(fen);
        let moves = pos.legal_moves();

        let h = pos.hash;
        for &mv in moves.iter() {
            pos.make_move(mv).unwrap();
            let h1 = pos.hash;
            pos.unmake_move(mv).unwrap();
            assert_ne!(h, h1);
        }
    }

    #[test]
    fn test_hash_similar() {
        let pairs = [
            ( // missing rook
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBN1 w KQkq - 0 1"
            ),
            ( // different player
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1"
            ),
            ( // different castling rights
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 1"
            ),

            ( // ep rights
                "rnbqkbnr/p1ppp1pp/5p2/1pP5/8/8/PP1PPPPP/RNBQKBNR w KQkq b6 0 3",
                "rnbqkbnr/p1ppp1pp/5p2/1pP5/8/8/PP1PPPPP/RNBQKBNR w KQkq - 0 3"
            ),
        ];

        for (fen1, fen2) in pairs.iter() {
            let pos1 = Position::from_fen(fen1);
            let pos2 = Position::from_fen(fen2);

            assert_ne!(pos1.hash, pos2.hash);
        }
    }
}