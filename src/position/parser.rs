#[path = "board.rs"]
pub mod board;

pub use board::*;

impl Position {
    pub fn from_fen(fen: &str) -> Self {
        let mut board = vec![EMPTY; FIELDS_NO];

        let parts: Vec<&str> = fen.split_ascii_whitespace().collect();

        let board_str = parts[0];
        let to_move_str = parts[1];
        let en_passant_str = parts[3];

        let castling_str = parts[2];
        let half_moves_str = parts[4];
        let full_moves_str = parts[5];

        let castling_rights = castling_str.to_string();
        let half_moves = half_moves_str.parse().unwrap();
        let full_moves = full_moves_str.parse().unwrap();

        let to_move = match to_move_str {
            "w" => Player::White,
            "b" => Player::Black,
            _ => panic!("Wrong player in FEN {}", to_move_str),
        };

        for (row, line) in board_str.split('/').rev().enumerate() {
            let mut col = 0;
            for ch in line.chars() {
                assert!(col < 8 && row < 8);
                if ch.is_ascii_digit() {
                    let val = ch.to_digit(10).unwrap();
                    col += val;
                } else {
                    let player = if ch.is_uppercase() {
                        Player::White
                    } else {
                        Player::Black
                    };
                    let piece_str = ch.to_lowercase().to_string().chars().nth(0).unwrap();
                    let piece = match piece_str {
                        'p' => Piece::Pawn,
                        'n' => Piece::Knight,
                        'b' => Piece::Bishop,
                        'r' => Piece::Rook,
                        'q' => Piece::Queen,
                        'k' => Piece::King,
                        _ => panic!("Wrong piece name in FEN {}", piece_str),
                    };
                    // let coord = board::rowcol2coord(row as i32, col as i32);
                    // println!(
                    //     "row {}, col {}, coord {} ==> {:?} of {:?}",
                    //     row, col, coord, piece, player
                    // );
                    let field = boardcell_encode(player, piece);
                    let index = rowcol2index(row as i32, col as i32);
                    board[index] = field;
                    col += 1;
                }
            }
        }

        let en_passant = if en_passant_str != "-" {
            let row_char = en_passant_str.chars().last().unwrap();
            if row_char != '3' && row_char != '6' {
                panic!(
                    "en_passant must end in 3 or 6, got {} instead",
                    en_passant_str
                );
            }
            let coord_str = en_passant_str.to_ascii_uppercase();

            Some(coord2index(str2coord(&coord_str).unwrap()))
        } else {
            None
        };

        Position::create(
            board,
            to_move,
            en_passant,
            castling_rights,
            half_moves,
            full_moves,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_initial_board_piece_colors() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let pos = Position::from_fen(fen);

        let rows = |player: Player| {
            if player == Player::White {
                vec![1, 2]
            } else {
                vec![7, 8]
            }
        };

        for &player in board::PLAYERS.iter() {
            for row in rows(player) {
                for col in "ABCDEFGH".chars() {
                    let coord = format!("{}{}", col, row);
                    assert!(pos[&coord] != EMPTY);
                    assert_eq!(player, boardcell_player(pos[&coord]));
                }
            }
        }
    }

    #[test]
    fn parse_initial_board_fields() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let pos = Position::from_fen(fen);

        let white_rook = boardcell_encode(Player::White, Piece::Rook);
        let white_king = boardcell_encode(Player::White, Piece::King);
        let white_queen = boardcell_encode(Player::White, Piece::Queen);

        let black_king = boardcell_encode(Player::Black, Piece::King);
        let black_queen = boardcell_encode(Player::Black, Piece::Queen);

        assert_eq!(white_rook, pos["A1"]);
        assert_eq!(white_queen, pos["D1"]);
        assert_eq!(white_king, pos["E1"]);

        assert_eq!(black_queen, pos["D8"]);
        assert_eq!(black_king, pos["E8"]);
    }

    #[test]
    fn parse_initial_board_en_passant() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let pos = Position::from_fen(fen);

        assert_eq!(None, pos.en_passant);
    }

    #[test]
    fn parse_initial_board_move_count() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let pos = Position::from_fen(fen);

        assert_eq!(1, pos.full_moves);
        assert_eq!(0, pos.half_moves);
    }

    #[test]
    fn parse_lonely_king() {
        let fen = "7K/8/8/8/8/8/8/8 w - - 0 1";
        let pos = Position::from_fen(fen);

        let white_king = boardcell_encode(Player::White, Piece::King);
        assert_eq!(white_king, pos["H8"]);
        assert_eq!("-", pos.castle_rights);
    }

    #[test]
    fn parse_en_passant() {
        let tests = [
            (
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
                None,
            ),
            (
                "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
                Some("E3"),
            ),
            (
                "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq e6 0 2",
                Some("E6"),
            ),
        ];

        for (fen, ep) in tests.iter() {
            let pos = Position::from_fen(fen);
            assert_eq!(ep.map(|c|coord2index(c)), pos.en_passant);
        }
    }

    #[test]
    fn parse_and_unparse() {
        let fens = [
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
            "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq e6 0 2",
            "r3k1n1/p6p/8/8/8/8/PPPP4/4KBNR b KQkq - 0 1",
            "r3k1n1/p6p/8/8/8/8/PPPP4/4KBNR b KQkq - 5 1",
            "r3k1n1/p6p/8/8/8/8/PPPP4/4KBNR b - - 5 1",
            "r3k1n1/p6p/8/8/8/8/PPPP4/4KBNR b kq - 5 1",
            "r3k1n1/p6p/8/8/8/8/PPPP4/4KBNR b Kk - 5 1",
            "r3k1n1/p6p/8/8/8/8/PPPP4/4KBNR b Qq - 5 1",
        ];

        for &fen in fens.iter() {
            let pos = Position::from_fen(fen);
            assert_eq!(fen, pos.to_fen());
        }
    }
}
