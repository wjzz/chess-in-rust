use rust_chess::{board, Position};

#[cfg(test)]
mod test_moves {

    use super::*;

    #[test]
    fn perf_starting_1() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

        let pos = Position::from_fen(fen);
        let moves = pos.moves();
        println!("perfm 1 = {:#?}", moves);

        assert_eq!(20, moves.len());
    }

    #[test]
    fn perf_starting_1_black() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1";

        let pos = Position::from_fen(fen);
        let moves = pos.moves();
        println!("perfm 1 = {:#?}", moves);

        assert_eq!(20, moves.len());
    }
}