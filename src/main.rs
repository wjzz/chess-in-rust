mod board;
mod parser;

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line = line.trim_end().to_string();

    let pos = parser::parse_fen(&line);
    println!("{}", pos.to_ascii());
    println!();

    let moves = pos.moves();
    println!("{:#?}", moves);
    println!("Total {} moves", moves.len());
}

#[cfg(test)]
mod test_moves {
    use super::*;

    #[test]
    fn moves_pawns_initial() {
        let fen = "8/8/8/8/8/8/PPPPPPPP/8 w KQkq - 0 1";
        let pos = parser::parse_fen(fen);
        let moves = pos.moves();

        assert_eq!(16, moves.len());
    }

    #[test]
    fn moves_pawns_second() {
        let fen = "8/8/8/8/8/PPPPPPPP/8/8 w KQkq - 0 1";
        let pos = parser::parse_fen(fen);
        let moves = pos.moves();

        assert_eq!(8, moves.len());
    }

    #[test]
    fn moves_pawns_promote_one() {
        let fen = "8/7P/8/8/8/8/8/8 w KQkq - 0 1";
        let pos = parser::parse_fen(fen);
        let moves = pos.moves();
        println!("moves = {:#?}", moves);

        assert_eq!(4, moves.len());
    }

    #[test]
    fn moves_pawns_promote_full() {
        let fen = "8/PPPPPPPP/8/8/8/8/8/8 w KQkq - 0 1";
        let pos = parser::parse_fen(fen);
        let moves = pos.moves();

        assert_eq!(8 * 4, moves.len());
    }

    #[test]
    fn moves_pawns_capture() {
        let fen = "8/8/p1p5/1P6/8/8/8/8 w KQkq - 0 1";
        let pos = parser::parse_fen(fen);
        let moves = pos.moves();

        assert_eq!(3, moves.len());
    }

    #[test]
    fn moves_pawns_capture_and_promote() {
        let fen = "p1p5/1P6/8/8/8/8/8/8 w KQkq - 0 1";
        let pos = parser::parse_fen(fen);
        let moves = pos.moves();

        assert_eq!(3 * 4, moves.len());
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
            let pos = parser::parse_fen(fen);
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
            let pos = parser::parse_fen(fen);
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
            let pos = parser::parse_fen(fen);
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
            let pos = parser::parse_fen(fen);
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
            let pos = parser::parse_fen(fen);
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
            let pos = parser::parse_fen(fen);
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
            let pos = parser::parse_fen(fen);
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
            let pos = parser::parse_fen(fen);
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
            let pos = parser::parse_fen(fen);
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
            let pos = parser::parse_fen(fen);
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
            let pos = parser::parse_fen(fen);
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
            let pos = parser::parse_fen(fen);
            let moves = pos.moves();
            println!("moves = {:#?}", moves);

            assert_eq!(21, moves.len());
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
            let pos = parser::parse_fen(fen);
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
            let pos = parser::parse_fen(fen);
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
            let pos = parser::parse_fen(fen);
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
            let pos = parser::parse_fen(fen);
            let moves = pos.moves();
            println!("moves = {:#?}", moves);

            assert_eq!(11, moves.len());
        }
    }


    #[test]
    fn perf_starting_1() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

        let pos = parser::parse_fen(fen);
        let moves = pos.moves();
        println!("perfm 1 = {:#?}", moves);

        assert_eq!(20, moves.len());
    }

    #[test]
    fn perf_starting_1_black() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1";

        let pos = parser::parse_fen(fen);
        let moves = pos.moves();
        println!("perfm 1 = {:#?}", moves);

        assert_eq!(20, moves.len());
    }
}
