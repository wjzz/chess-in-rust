use rust_chess::{board, Position};

#[cfg(test)]
mod test_moves {

    use super::*;

    #[test]
    fn moves_pawns_initial() {
        let fen = "8/8/8/8/8/8/PPPPPPPP/8 w KQkq - 0 1";
        let pos = Position::from_fen(fen);
        let moves = pos.moves();

        assert_eq!(16, moves.len());
    }

    #[test]
    fn moves_pawns_second() {
        let fen = "8/8/8/8/8/PPPPPPPP/8/8 w KQkq - 0 1";
        let pos = Position::from_fen(fen);
        let moves = pos.moves();

        assert_eq!(8, moves.len());
    }

    #[test]
    fn moves_pawns_promote_one() {
        let fen = "8/7P/8/8/8/8/8/8 w KQkq - 0 1";
        let pos = Position::from_fen(fen);
        let moves = pos.moves();
        println!("moves = {:#?}", moves);

        assert_eq!(4, moves.len());
    }

    #[test]
    fn moves_pawns_promote_full() {
        let fen = "8/PPPPPPPP/8/8/8/8/8/8 w KQkq - 0 1";
        let pos = Position::from_fen(fen);
        let moves = pos.moves();

        assert_eq!(8 * 4, moves.len());
    }

    #[test]
    fn moves_pawns_capture() {
        let fen = "8/8/p1p5/1P6/8/8/8/8 w KQkq - 0 1";
        let pos = Position::from_fen(fen);
        let moves = pos.moves();

        assert_eq!(3, moves.len());
    }

    #[test]
    fn moves_pawns_capture_and_promote() {
        let fen = "p1p5/1P6/8/8/8/8/8/8 w KQkq - 0 1";
        let pos = Position::from_fen(fen);
        let moves = pos.moves();

        assert_eq!(3 * 4, moves.len());
    }

    #[test]
    fn moves_pawns_no_capture_forward() {
        let fen = "8/8/1p6/1P6/8/8/8/8 w KQkq - 0 1";
        let pos = Position::from_fen(fen);
        let moves = pos.moves();

        assert_eq!(0, moves.len());
    }

    #[test]
    fn moves_pawns_en_passant() {
        let fen = "8/8/8/8/4Pp2/8/8/8 b - e3 0 1";
        let pos = Position::from_fen(fen);
        let moves = pos.moves();

        assert_eq!(2, moves.len());
    }

    #[test]
    fn moves_king_corners() {
        let fens = [
            "K7/8/8/8/8/8/8/8 w KQkq - 0 1",
            "7K/8/8/8/8/8/8/8 w KQkq - 0 1",
            "8/8/8/8/8/8/8/K7 w KQkq - 0 1",
            "8/8/8/8/8/8/8/7K w KQkq - 0 1",
        ];

        for &fen in fens.iter() {
            let pos = Position::from_fen(fen);
            let moves = pos.moves();
            println!("moves = {:#?}", moves);

            assert_eq!(3, moves.len());
        }
    }

    #[test]
    fn moves_king_edges() {
        let fens = [
            "3K4/8/8/8/8/8/8/8 w KQkq - 0 1",
            "8/8/8/K7/8/8/8/8 w KQkq - 0 1",
            "8/8/8/7K/8/8/8/8 w KQkq - 0 1",
            "8/8/8/8/8/8/8/5K2 w KQkq - 0 1",
        ];

        for &fen in fens.iter() {
            let pos = Position::from_fen(fen);
            let moves = pos.moves();
            println!("moves = {:#?}", moves);

            assert_eq!(5, moves.len());
        }
    }

    #[test]
    fn moves_king_inside() {
        let fens = [
            "8/3K4/8/8/8/8/8/8 w KQkq - 0 1",
            "8/8/8/5K2/8/8/8/8 w KQkq - 0 1",
            "8/8/8/8/8/8/1K6/8 w KQkq - 0 1",
            "8/8/8/8/8/4K3/8/8 w KQkq - 0 1",
        ];

        for &fen in fens.iter() {
            let pos = Position::from_fen(fen);
            let moves = pos.moves();
            println!("moves = {:#?}", moves);

            assert_eq!(8, moves.len());
        }
    }

    #[test]
    fn moves_knight_corners() {
        let fens = [
            "N7/8/8/8/8/8/8/8 w KQkq - 0 1",
            "7N/8/8/8/8/8/8/8 w KQkq - 0 1",
            "8/8/8/8/8/8/8/N7 w KQkq - 0 1",
            "8/8/8/8/8/8/8/7N w KQkq - 0 1",
        ];

        for &fen in fens.iter() {
            let pos = Position::from_fen(fen);
            let moves = pos.moves();
            println!("moves = {:#?}", moves);

            assert_eq!(2, moves.len());
        }
    }

    #[test]
    fn moves_knight_edges() {
        let fens = [
            "3N4/8/8/8/8/8/8/8 w KQkq - 0 1",
            "8/8/8/N7/8/8/8/8 w KQkq - 0 1",
            "8/8/8/7N/8/8/8/8 w KQkq - 0 1",
            "8/8/8/8/8/8/8/5N2 w KQkq - 0 1",
        ];

        for &fen in fens.iter() {
            let pos = Position::from_fen(fen);
            let moves = pos.moves();
            println!("moves = {:#?}", moves);

            assert_eq!(4, moves.len());
        }
    }

    #[test]
    fn moves_knight_one_step_from_edge() {
        let fens = [
            "8/8/1N6/8/8/8/8/8 w KQkq - 0 1",
            "8/8/8/8/8/6N1/8/8 w KQkq - 0 1",
            "8/8/8/8/8/1N6/8/8 w KQkq - 0 1",
            "8/8/8/8/6N1/8/8/8 w KQkq - 0 1",
        ];

        for &fen in fens.iter() {
            let pos = Position::from_fen(fen);
            let moves = pos.moves();
            println!("moves = {:#?}", moves);

            assert_eq!(6, moves.len());
        }
    }

    #[test]
    fn moves_knight_inside() {
        let fens = [
            "8/8/2N5/8/8/8/8/8 w KQkq - 0 1",
            "8/8/8/8/8/4N3/8/8 w KQkq - 0 1",
            "8/8/8/8/8/3N4/8/8 w KQkq - 0 1",
            "8/8/8/8/5N2/8/8/8 w KQkq - 0 1",
        ];

        for &fen in fens.iter() {
            let pos = Position::from_fen(fen);
            let moves = pos.moves();
            println!("moves = {:#?}", moves);

            assert_eq!(8, moves.len());
        }
    }

    #[test]
    fn moves_rook() {
        let fens = [
            "R7/8/8/8/8/8/8/8 w KQkq - 0 1",
            "4R3/8/8/8/8/8/8/8 w KQkq - 0 1",
            "7R/8/8/8/8/8/8/8 w KQkq - 0 1",
            "8/8/R7/8/8/8/8/8 w KQkq - 0 1",
            "8/8/4R3/8/8/8/8/8 w KQkq - 0 1",
            "8/8/7R/8/8/8/8/8 w KQkq - 0 1",
            "8/8/8/8/8/8/8/R7 w KQkq - 0 1",
            "8/8/8/8/8/8/8/4R3 w KQkq - 0 1",
            "8/8/8/8/8/8/8/7R w KQkq - 0 1",
        ];

        for &fen in fens.iter() {
            let pos = Position::from_fen(fen);
            let moves = pos.moves();
            println!("moves = {:#?}", moves);

            assert_eq!(14, moves.len());
        }
    }

    #[test]
    fn moves_queen_corner() {
        let fens = [
            "Q7/8/8/8/8/8/8/8 w KQkq - 0 1",
            "7Q/8/8/8/8/8/8/8 w KQkq - 0 1",
            "8/8/8/8/8/8/8/Q7 w KQkq - 0 1",
            "8/8/8/8/8/8/8/7Q w KQkq - 0 1",
        ];

        for &fen in fens.iter() {
            let pos = Position::from_fen(fen);
            let moves = pos.moves();
            println!("moves = {:#?}", moves);

            assert_eq!(21, moves.len());
        }
    }

    #[test]
    fn moves_queen_inside() {
        let fens = [
            "8/8/8/8/4Q3/8/8/8 w KQkq - 0 1",
            "8/8/8/3Q4/8/8/8/8 w KQkq - 0 1",
        ];

        for &fen in fens.iter() {
            let pos = Position::from_fen(fen);
            let moves = pos.moves();
            println!("moves = {:#?}", moves);

            assert_eq!(27, moves.len());
        }
    }

    #[test]
    fn moves_queen_inside2() {
        let fens = [
            "8/8/8/8/8/5Q2/8/8 w KQkq - 0 1",
            "8/8/2Q5/8/8/8/8/8 w KQkq - 0 1",
        ];

        for &fen in fens.iter() {
            let pos = Position::from_fen(fen);
            let moves = pos.moves();
            println!("moves = {:#?}", moves);

            assert_eq!(25, moves.len());
        }
    }

    #[test]
    fn moves_queen_edges() {
        let fens = [
            "3Q4/8/8/8/8/8/8/8 w KQkq - 0 1",
            "8/8/8/Q7/8/8/8/8 w KQkq - 0 1",
            "8/8/8/7Q/8/8/8/8 w KQkq - 0 1",
            "8/8/8/8/8/8/8/5Q2 w KQkq - 0 1",
        ];

        for &fen in fens.iter() {
            let pos = Position::from_fen(fen);
            let moves = pos.moves();
            println!("moves = {:#?}", moves);

            assert_eq!(21, moves.len());
        }
    }

    #[test]
    fn moves_queen_blocked() {
        let inputs = [
            ("Qp6/pp6/8/8/8/8/8/8 w KQkq - 0 1", 3),
            (
                "rnbqkbnr/pppppppp/8/8/8/3P4/PPP1PPPP/RNBQKBNR w KQkq - 0 1",
                27,
            ),
            (
                "rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 1",
                28,
            ),
        ];

        for (fen, value) in inputs.iter() {
            let pos = Position::from_fen(fen);
            let moves = pos.moves();
            println!("moves = {:#?}", moves);

            assert_eq!(*value, moves.len());
        }
    }

    #[test]
    fn moves_bishop_corner() {
        let fens = [
            "B7/8/8/8/8/8/8/8 w KQkq - 0 1",
            "7B/8/8/8/8/8/8/8 w KQkq - 0 1",
            "8/8/8/8/8/8/8/B7 w KQkq - 0 1",
            "8/8/8/8/8/8/8/7B w KQkq - 0 1",
        ];

        for &fen in fens.iter() {
            let pos = Position::from_fen(fen);
            let moves = pos.moves();
            println!("moves = {:#?}", moves);

            assert_eq!(7, moves.len());
        }
    }

    #[test]
    fn moves_bishop_edges() {
        let fens = [
            "3B4/8/8/8/8/8/8/8 w KQkq - 0 1",
            "8/8/8/B7/8/8/8/8 w KQkq - 0 1",
            "8/8/8/7B/8/8/8/8 w KQkq - 0 1",
            "8/8/8/8/8/8/8/5B2 w KQkq - 0 1",
        ];

        for &fen in fens.iter() {
            let pos = Position::from_fen(fen);
            let moves = pos.moves();
            println!("moves = {:#?}", moves);

            assert_eq!(7, moves.len());
        }
    }

    #[test]
    fn moves_bishop_inside() {
        let fens = [
            "8/8/8/8/4B3/8/8/8 w KQkq - 0 1",
            "8/8/8/3B4/8/8/8/8 w KQkq - 0 1",
        ];

        for &fen in fens.iter() {
            let pos = Position::from_fen(fen);
            let moves = pos.moves();
            println!("moves = {:#?}", moves);

            assert_eq!(13, moves.len());
        }
    }

    #[test]
    fn moves_bishop_inside2() {
        let fens = [
            "8/8/8/8/8/5B2/8/8 w KQkq - 0 1",
            "8/8/2B5/8/8/8/8/8 w KQkq - 0 1",
        ];

        for &fen in fens.iter() {
            let pos = Position::from_fen(fen);
            let moves = pos.moves();
            println!("moves = {:#?}", moves);

            assert_eq!(11, moves.len());
        }
    }

    #[test]
    fn not_in_check() {
        let player = board::Player::White;
        let fens = [
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            "8/8/8/8/8/8/8/4K3 w KQkq - 0 1",
        ];

        for &fen in fens.iter() {
            let pos = Position::from_fen(fen);
            assert_eq!(false, pos.is_king_in_check(player));
        }
    }

    #[test]
    fn in_check() {
        let player = board::Player::White;
        let fens = [
            "8/8/8/8/8/8/8/r3K3 w KQkq - 0 1",
            "7b/8/8/8/8/8/8/K7 w KQkq - 0 1",
            "8/8/8/8/8/8/1p6/K7 w KQkq - 0 1",
            "8/8/8/8/8/1n6/8/K7 w KQkq - 0 1",
        ];

        for &fen in fens.iter() {
            let pos = Position::from_fen(fen);
            assert_eq!(true, pos.is_king_in_check(player));
        }
    }

    #[test]
    fn filter_moves_leaving_king_in_check() {
        let inputs = [(
            "rnbqkbnr/ppppp1pp/5p2/7Q/4P3/8/PPPP1PPP/RNB1KBNR b KQkq - 1 2",
            1,
        )];

        for (fen, value) in inputs.iter() {
            let pos = Position::from_fen(fen);
            let moves = pos.legal_moves();
            println!("moves = {:#?}", moves);

            assert_eq!(*value, moves.len());
        }
    }
}
