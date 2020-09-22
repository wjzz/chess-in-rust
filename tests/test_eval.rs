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
            pos.make_move(mv);
            let ev = pos.eval();
            pos.unmake_move(mv);
            println!("{} => {:+.1}", intmove_to_uci_ascii(mv), ev);
        }

        assert_eq!(20, moves.len());
    }
}