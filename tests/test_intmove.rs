use rust_chess::Position;
use rust_chess::*;

#[cfg(test)]
mod test_moves {

    use super::*;

    #[test]
    fn test_conv() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let mut pos = Position::from_fen(fen);

        let moves = pos.legal_moves();

        for mv in moves.iter() {
            let mv2 = intmove_from_move(&move_from_intmove(*mv));
            assert_eq!(*mv, mv2);
        }
    }
}
