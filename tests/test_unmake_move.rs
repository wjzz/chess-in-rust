use rust_chess::*;

#[cfg(test)]
mod test_unmoves {

    use super::*;

    #[test]
    fn unmake_move_starting() {
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
                "rnbqkbnr/pppppppp/8/8/8/N7/PPPPPPPP/R1BQKBNR b KQkq - 1 1",
            ),
            (
                "B1->C3",
                "rnbqkbnr/pppppppp/8/8/8/2N5/PPPPPPPP/R1BQKBNR b KQkq - 1 1",
            ),
            (
                "G1->F3",
                "rnbqkbnr/pppppppp/8/8/8/5N2/PPPPPPPP/RNBQKB1R b KQkq - 1 1",
            ),
            (
                "G1->H3",
                "rnbqkbnr/pppppppp/8/8/8/7N/PPPPPPPP/RNBQKB1R b KQkq - 1 1",
            ),
        ];

        for (mv, fen_after) in inputs.iter() {
            let mut pos = Position::from_fen(fen);
            let mv = Move::from_ascii(mv);
            pos.make_move(mv).unwrap();
            assert_eq!(fen_after.to_string(), pos.to_fen());

            pos.unmake_move(mv).unwrap();
            assert_eq!(fen, pos.to_fen());
        }
    }


    #[test]
    fn unmake_move_starting2() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

        let mut pos = Position::from_fen(fen);
        let moves = pos.legal_moves();

        assert_eq!(fen, pos.to_fen());
        for mv in moves.iter() {
            pos.make_move(*mv).unwrap();
            pos.unmake_move(*mv).unwrap();
            assert_eq!(fen, pos.to_fen());
        }
    }

    #[test]
    fn unmake_move_starting_depth2() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

        let mut pos = Position::from_fen(fen);
        let moves = pos.legal_moves();

        assert_eq!(fen, pos.to_fen());
        for mv in moves.iter() {
            pos.make_move(*mv).unwrap();
            let moves2 = pos.legal_moves();
            for mv2 in moves2.iter() {
                pos.make_move(*mv2).unwrap();
                pos.unmake_move(*mv2).unwrap();
            }
            pos.unmake_move(*mv).unwrap();
            assert_eq!(fen, pos.to_fen());
        }
    }

    #[test]
    fn unmake_move_starting_depth3() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

        let mut pos = Position::from_fen(fen);
        let moves = pos.legal_moves();

        assert_eq!(fen, pos.to_fen());
        for mv in moves.iter() {
            pos.make_move(*mv).unwrap();

            let before = pos.to_fen();

            let moves2 = pos.legal_moves();
            for mv2 in moves2.iter() {
                let before2 = pos.to_fen();

                pos.make_move(*mv2).unwrap();
                let moves3 = pos.legal_moves();
                for mv3 in moves3.iter() {
                    let before3 = pos.to_fen();

                    pos.make_move(*mv3).unwrap();
                    pos.unmake_move(*mv3).unwrap();

                    let after3 = pos.to_fen();
                    assert_eq!(before3, after3);
                }
                pos.unmake_move(*mv2).unwrap();
                let after2 = pos.to_fen();
                if after2 != before2 {
                    println!("move {}", mv.to_usi_ascii());
                    println!("move {}", mv2.to_usi_ascii());
                }
                assert_eq!(before2, after2);
            }
            let after = pos.to_fen();
            assert_eq!(before, after);

            pos.unmake_move(*mv).unwrap();
            assert_eq!(fen, pos.to_fen());
        }
    }


    #[test]
    fn unmake_move_change_colors() {
        use Player::*;

        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let mut pos = Position::from_fen(fen);
        assert_eq!(White, pos.to_move);

        let mv = Move::from_ascii("E2->E4");
        pos.make_move(mv).unwrap();
        assert_eq!(Black, pos.to_move);

        let mv2 = Move::from_ascii("E7->E5");
        pos.make_move(mv2).unwrap();
        assert_eq!(White, pos.to_move);

        pos.unmake_move(mv2).unwrap();
        assert_eq!(Black, pos.to_move);

        pos.unmake_move(mv).unwrap();
        assert_eq!(White, pos.to_move);
    }

    #[test]
    fn unmake_move_change_full_moves() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let mut pos = Position::from_fen(fen);
        assert_eq!(1, pos.full_moves);

        // white moves
        let mv = Move::from_ascii("E2->E4");
        pos.make_move(mv).unwrap();
        assert_eq!(1, pos.full_moves);

        // black moves
        let mv2 = Move::from_ascii("E7->E5");
        pos.make_move(mv2).unwrap();
        assert_eq!(2, pos.full_moves);

        // white moves
        let mv3 = Move::from_ascii("D2->D4");
        pos.make_move(mv3).unwrap();
        assert_eq!(2, pos.full_moves);

        // black moves
        let mv4 = Move::from_ascii("D7->D5");
        pos.make_move(mv4).unwrap();
        assert_eq!(3, pos.full_moves);

        // black move unmake
        pos.unmake_move(mv4).unwrap();
        assert_eq!(2, pos.full_moves);

        // white move unmake
        pos.unmake_move(mv3).unwrap();
        assert_eq!(2, pos.full_moves);

        // black move unmake
        pos.unmake_move(mv2).unwrap();
        assert_eq!(1, pos.full_moves);

        // white move unmake
        pos.unmake_move(mv).unwrap();
        assert_eq!(1, pos.full_moves);
    }

    #[test]
    fn unmake_move_change_half_moves() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let mut pos = Position::from_fen(fen);
        assert_eq!(0, pos.half_moves);

        // white moves a pawn
        let mv = Move::from_ascii("E2->E4");
        pos.make_move(mv).unwrap();
        assert_eq!(0, pos.half_moves);

        // black moves a pawn too
        let mv2 = Move::from_ascii("E7->E5");
        pos.make_move(mv2).unwrap();
        assert_eq!(0, pos.half_moves);

        // white moves a knight
        let mv3 = Move::from_ascii("G1->F3");
        pos.make_move(mv3).unwrap();
        assert_eq!(1, pos.half_moves);

        // black moves a knight too
        let mv4 = Move::from_ascii("B8->C6");
        pos.make_move(mv4).unwrap();
        assert_eq!(2, pos.half_moves);

        // white captures a pawn by the knight
        let mv5 = Move::from_ascii("F3->E5");
        pos.make_move(mv5).unwrap();
        assert_eq!(0, pos.half_moves);

        // white undoes the knight capturing a pawn
        pos.unmake_move(mv5).unwrap();
        assert_eq!(2, pos.half_moves);

        // black unmoves a knight too
        pos.unmake_move(mv4).unwrap();
        assert_eq!(1, pos.half_moves);

        // white unmoves a knight
        pos.unmake_move(mv3).unwrap();
        assert_eq!(0, pos.half_moves);

        pos.unmake_move(mv2).unwrap();
        assert_eq!(0, pos.half_moves);

        pos.unmake_move(mv).unwrap();
        assert_eq!(0, pos.half_moves);
    }

    #[test]
    fn unmake_move_starting_knight() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let mut pos = Position::from_fen(fen);

        let mv = "B1->C3"; // knight
        pos.make_move(Move::from_ascii(mv)).unwrap();

        let result_fen = "rnbqkbnr/pppppppp/8/8/8/2N5/PPPPPPPP/R1BQKBNR b KQkq - 1 1";
        assert_eq!(pos.to_fen(), result_fen);

        pos.unmake_move(Move::from_ascii(mv)).unwrap();
        assert_eq!(pos.to_fen(), fen);
    }


    #[test]
    fn unmake_move_starting_ep1() {
        let fen = "k7/8/8/4pP2/8/8/8/K7 w KQkq e6 0 1";
        let mut pos = Position::from_fen(fen);

        let mv = "F5->E6"; // ep.
        pos.make_move(Move::from_ascii(mv)).unwrap();

        let result_fen = "k7/8/4P3/8/8/8/8/K7 b KQkq - 0 1";
        assert_eq!(pos.to_fen(), result_fen);

        pos.unmake_move(Move::from_ascii(mv)).unwrap();
        assert_eq!(pos.to_fen(), fen);
    }

    #[test]
    fn unmake_move_starting_ep() {
        let fen = "8/8/8/8/4Pp2/8/8/8 b KQkq e3 0 1";
        let mut pos = Position::from_fen(fen);

        let mv = "F4->E3"; // ep.
        pos.make_move(Move::from_ascii(mv)).unwrap();

        let result_fen = "8/8/8/8/8/4p3/8/8 w KQkq - 0 2";
        assert_eq!(pos.to_fen(), result_fen);

        pos.unmake_move(Move::from_ascii(mv)).unwrap();
        assert_eq!(pos.to_fen(), fen);
    }

    #[test]
    fn unmake_move_lose_castling_rights_left_rook() {
        let fen = "6k1/8/8/8/8/8/8/R3K2R w KQ - 0 1";
        let mut pos = Position::from_fen(fen);

        assert!(pos.castle_rights.contains("K"));
        assert!(pos.castle_rights.contains("Q"));

        let mv = "A1->A2";
        pos.make_move(Move::from_ascii(mv)).unwrap();

        let result_fen = "6k1/8/8/8/8/8/R7/4K2R b K - 1 1";

        assert_eq!(pos.to_fen(), result_fen);
        assert!(pos.castle_rights.contains("K"));
        assert!(!pos.castle_rights.contains("Q"));

        pos.unmake_move(Move::from_ascii(mv)).unwrap();
        assert_eq!(pos.to_fen(), fen);
    }

    #[test]
    fn unmake_move_lose_castling_rights_right_rook() {
        let fen = "6k1/8/8/8/8/8/8/R3K2R w KQ - 0 1";
        let mut pos = Position::from_fen(fen);

        assert!(pos.castle_rights.contains("K"));
        assert!(pos.castle_rights.contains("Q"));

        let mv = "H1->H2";
        pos.make_move(Move::from_ascii(mv)).unwrap();

        let result_fen = "6k1/8/8/8/8/8/7R/R3K3 b Q - 1 1";

        assert_eq!(pos.to_fen(), result_fen);
        assert!(!pos.castle_rights.contains("K"));
        assert!(pos.castle_rights.contains("Q"));

        pos.unmake_move(Move::from_ascii(mv)).unwrap();
        assert_eq!(pos.to_fen(), fen);
    }

    #[test]
    fn unmake_move_lose_castling_rights_king() {
        let fen = "6k1/8/8/8/8/8/8/R3K2R w KQ - 0 1";
        let mut pos = Position::from_fen(fen);

        assert!(pos.castle_rights.contains("K"));
        assert!(pos.castle_rights.contains("Q"));

        let mv = "E1->E2";
        pos.make_move(Move::from_ascii(mv)).unwrap();

        let result_fen = "6k1/8/8/8/8/8/4K3/R6R b - - 1 1";

        assert_eq!(pos.to_fen(), result_fen);
        assert!(!pos.castle_rights.contains("K"));
        assert!(!pos.castle_rights.contains("Q"));

        pos.unmake_move(Move::from_ascii(mv)).unwrap();
        assert_eq!(pos.to_fen(), fen);
    }

    #[test]
    fn unmake_move_kingside_castling() {
        let fen = "6k1/8/8/8/8/8/8/R3K2R w KQ - 0 1";
        let mut pos = Position::from_fen(fen);

        assert!(pos.castle_rights.contains("K"));
        assert!(pos.castle_rights.contains("Q"));

        let mv = "E1->G1";
        pos.make_move(Move::from_ascii(mv)).unwrap();

        let result_fen = "6k1/8/8/8/8/8/8/R4RK1 b - - 1 1";
        assert_eq!(pos.to_fen(), result_fen);
        assert!(!pos.castle_rights.contains("K"));
        assert!(!pos.castle_rights.contains("Q"));

        pos.unmake_move(Move::from_ascii(mv)).unwrap();
        assert_eq!(pos.to_fen(), fen);
    }

    #[test]
    fn unmake_move_queenside_castling() {
        let fen = "6k1/8/8/8/8/8/8/R3K2R w KQ - 0 1";
        let mut pos = Position::from_fen(fen);

        assert!(pos.castle_rights.contains("K"));
        assert!(pos.castle_rights.contains("Q"));

        let mv = "E1->C1";
        pos.make_move(Move::from_ascii(mv)).unwrap();

        let result_fen = "6k1/8/8/8/8/8/8/2KR3R b - - 1 1";
        assert_eq!(pos.to_fen(), result_fen);
        assert!(!pos.castle_rights.contains("K"));
        assert!(!pos.castle_rights.contains("Q"));

        pos.unmake_move(Move::from_ascii(mv)).unwrap();
        assert_eq!(pos.to_fen(), fen);
    }

    #[test]
    fn unmake_move_losing_the_rook_loses_castling() {
        let fen = "rnbqk1nr/pppppp1p/6pb/8/8/1P6/PBPPPPPP/RN1QKBNR w KQkq - 2 3";
        let mut pos = Position::from_fen(fen);

        assert!(pos.castle_rights.contains("k"));
        assert!(pos.castle_rights.contains("q"));

        let mv = "B2->H8";
        pos.make_move(Move::from_ascii(mv)).unwrap();

        let result_fen = "rnbqk1nB/pppppp1p/6pb/8/8/1P6/P1PPPPPP/RN1QKBNR b KQq - 0 3";
        assert_eq!(pos.to_fen(), result_fen);
        assert!(!pos.castle_rights.contains("k"));
        assert!(pos.castle_rights.contains("q"));

        pos.unmake_move(Move::from_ascii(mv)).unwrap();
        assert_eq!(pos.to_fen(), fen);
    }

    #[test]
    fn unmake_move_losing_the_rook_loses_castling2() {
        let fen = "rn1qkbnr/p1pppppp/bp6/8/8/6P1/PPPPPPBP/RNBQK1NR w KQkq - 2 3";
        let mut pos = Position::from_fen(fen);

        assert!(pos.castle_rights.contains("k"));
        assert!(pos.castle_rights.contains("q"));

        let mv = "G2->A8";
        pos.make_move(Move::from_ascii(mv)).unwrap();

        let result_fen = "Bn1qkbnr/p1pppppp/bp6/8/8/6P1/PPPPPP1P/RNBQK1NR b KQk - 0 3";
        assert_eq!(pos.to_fen(), result_fen);
        assert!(pos.castle_rights.contains("k"));
        assert!(!pos.castle_rights.contains("q"));

        pos.unmake_move(Move::from_ascii(mv)).unwrap();
        assert_eq!(pos.to_fen(), fen);
    }
}
