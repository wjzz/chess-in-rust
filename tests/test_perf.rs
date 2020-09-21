use rust_chess::Position;

#[cfg(test)]
mod test_moves {

    use super::*;

    #[test]
    fn perf_starting_1() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

        let mut pos = Position::from_fen(fen);
        let moves = pos.legal_moves();
        println!("perfm 1 = {:#?}", moves);

        assert_eq!(20, moves.len());
    }

    #[test]
    fn perf_starting_1_black() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1";

        let mut pos = Position::from_fen(fen);
        let moves = pos.legal_moves();
        println!("perfm 1 = {:#?}", moves);

        assert_eq!(20, moves.len());
    }


    #[test]
    fn perf_no_castling_queenside_pawn_attack() {
        let fen = "r3k3/p1ppPpb1/bn2pnp1/4N2r/1p2P3/2N2Q1p/PPPBBPPP/R3K2R b KQq - 0 2";

        let mut pos = Position::from_fen(fen);
        let moves = pos.legal_moves();
        println!("perfm 1 = {:#?}", moves);

        assert_eq!(37, moves.len());
    }

    #[test]
    #[ignore]
    fn perf_imm_starting_1() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

        let expected = [
            20, 400, 8902, 197281,
        ];

        for (i, &value) in expected.iter().enumerate() {
            let i = i as u32;
            let result = Position::perft_immutable(i + 1, fen);
            println!("perf imm {} = {:#?}", i + 1, result);

            assert_eq!(value, result);
        }
    }

    // http://cinnamonchess.altervista.org/perft.html
    #[test]
    #[ignore]
    fn perf_imm_example_1() {
        let fen = "8/PPP4k/8/8/8/8/4Kppp/8 w - - 0 1";

        let expected = [18, 290, 5044, 89363];

        for (i, &value) in expected.iter().enumerate() {
            let i = i as u32;
            let result = Position::perft_immutable(i + 1, fen);
            println!("perf imm {} = {:#?}", i + 1, result);

            assert_eq!(value, result);
        }
    }
}
