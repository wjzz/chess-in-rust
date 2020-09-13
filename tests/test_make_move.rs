use rust_chess::*;

#[cfg(test)]
mod test_moves {

    use super::*;

    #[test]
    fn make_move_starting() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

        let inputs = [
            // pawns (2 squares)
            (
                "A2->A4",
                "rnbqkbnr/pppppppp/8/8/P7/8/1PPPPPPP/RNBQKBNR b KQkq a3 0 1",
            ),
            (
                "B2->B4",
                "rnbqkbnr/pppppppp/8/8/1P6/8/P1PPPPPP/RNBQKBNR b KQkq b3 0 1",
            ),
            (
                "C2->C4",
                "rnbqkbnr/pppppppp/8/8/2P5/8/PP1PPPPP/RNBQKBNR b KQkq c3 0 1",
            ),
            (
                "D2->D4",
                "rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq d3 0 1",
            ),
            (
                "E2->E4",
                "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
            ),
            (
                "F2->F4",
                "rnbqkbnr/pppppppp/8/8/5P2/8/PPPPP1PP/RNBQKBNR b KQkq f3 0 1",
            ),
            (
                "G2->G4",
                "rnbqkbnr/pppppppp/8/8/6P1/8/PPPPPP1P/RNBQKBNR b KQkq g3 0 1",
            ),
            (
                "H2->H4",
                "rnbqkbnr/pppppppp/8/8/7P/8/PPPPPPP1/RNBQKBNR b KQkq h3 0 1",
            ),
            // pawns (1 square)
            (
                "A2->A3",
                "rnbqkbnr/pppppppp/8/8/8/P7/1PPPPPPP/RNBQKBNR b KQkq - 0 1",
            ),
            (
                "B2->B3",
                "rnbqkbnr/pppppppp/8/8/8/1P6/P1PPPPPP/RNBQKBNR b KQkq - 0 1",
            ),
            (
                "C2->C3",
                "rnbqkbnr/pppppppp/8/8/8/2P5/PP1PPPPP/RNBQKBNR b KQkq - 0 1",
            ),
            (
                "D2->D3",
                "rnbqkbnr/pppppppp/8/8/8/3P4/PPP1PPPP/RNBQKBNR b KQkq - 0 1",
            ),
            (
                "E2->E3",
                "rnbqkbnr/pppppppp/8/8/8/4P3/PPPP1PPP/RNBQKBNR b KQkq - 0 1",
            ),
            (
                "F2->F3",
                "rnbqkbnr/pppppppp/8/8/8/5P2/PPPPP1PP/RNBQKBNR b KQkq - 0 1",
            ),
            (
                "G2->G3",
                "rnbqkbnr/pppppppp/8/8/8/6P1/PPPPPP1P/RNBQKBNR b KQkq - 0 1",
            ),
            (
                "H2->H3",
                "rnbqkbnr/pppppppp/8/8/8/7P/PPPPPPP1/RNBQKBNR b KQkq - 0 1",
            ),
            // knights
            (
                "B1->A3",
                "rnbqkbnr/pppppppp/8/8/8/N7/PPPPPPPP/R1BQKBNR b KQkq - 0 1",
            ),
            (
                "B1->C3",
                "rnbqkbnr/pppppppp/8/8/8/2N5/PPPPPPPP/R1BQKBNR b KQkq - 0 1",
            ),
            (
                "G1->F3",
                "rnbqkbnr/pppppppp/8/8/8/5N2/PPPPPPPP/RNBQKB1R b KQkq - 0 1",
            ),
            (
                "G1->H3",
                "rnbqkbnr/pppppppp/8/8/8/7N/PPPPPPPP/RNBQKB1R b KQkq - 0 1",
            ),
        ];

        for (mv, fen_after) in inputs.iter() {
            let mut pos = Position::from_fen(fen);
            let mv = Move::from_ascii(mv);
            pos.make_move(mv).unwrap();
            assert_eq!(fen_after.to_string(), pos.to_fen());
        }
    }

    #[test]
    fn make_move_change_colors() {
        use Player::*;

        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let mut pos = Position::from_fen(fen);
        assert_eq!(White, pos.to_move);

        let mv = Move::from_ascii("E2->E4");
        pos.make_move(mv).unwrap();
        assert_eq!(Black, pos.to_move);

        let mv = Move::from_ascii("E7->E5");
        pos.make_move(mv).unwrap();
        assert_eq!(White, pos.to_move);
    }

    #[test]
    fn make_move_change_full_moves() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let mut pos = Position::from_fen(fen);
        assert_eq!(1, pos.full_moves);

        // white moves
        let mv = Move::from_ascii("E2->E4");
        pos.make_move(mv).unwrap();
        assert_eq!(1, pos.full_moves);

        // black moves
        let mv = Move::from_ascii("E7->E5");
        pos.make_move(mv).unwrap();
        assert_eq!(2, pos.full_moves);

        // white moves
        let mv = Move::from_ascii("D2->D4");
        pos.make_move(mv).unwrap();
        assert_eq!(2, pos.full_moves);

        // black moves
        let mv = Move::from_ascii("D7->D5");
        pos.make_move(mv).unwrap();
        assert_eq!(3, pos.full_moves);
    }

    #[test]
    fn make_move_starting_ep1() {
        let fen = "k7/8/8/4pP2/8/8/8/K7 w KQkq e6 0 1";
        let mut pos = Position::from_fen(fen);

        let mv = "F5->E6"; // ep.
        pos.make_move(Move::from_ascii(mv)).unwrap();

        let result_fen = "k7/8/4P3/8/8/8/8/K7 b KQkq - 0 1";
        assert_eq!(pos.to_fen(), result_fen);
    }

    #[test]
    fn make_move_starting_ep() {
        let fen = "8/8/8/8/4Pp2/8/8/8 b KQkq e3 0 1";
        let mut pos = Position::from_fen(fen);

        let mv = "F4->E3"; // ep.
        pos.make_move(Move::from_ascii(mv)).unwrap();

        let result_fen = "8/8/8/8/8/4p3/8/8 w KQkq - 0 2";
        assert_eq!(pos.to_fen(), result_fen);
    }
}
