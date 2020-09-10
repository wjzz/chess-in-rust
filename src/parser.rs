#[path = "board.rs"]
mod board;

pub use board::{rowcol2index, Piece, Player, PlayerPiece, Position, FIELDS_NO};

pub fn parse_fen(fen: &str) -> Position {
    let mut board = vec![None; FIELDS_NO];

    let parts: Vec<&str> = fen.split_ascii_whitespace().collect();

    let board_str = parts[0];
    let to_move_str = parts[1];
    let _castling_str = parts[2];
    // TODO: Check the meaning of parts 3,4,5

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
                let coord = board::rowcol2coord(row as i32, col as i32);
                println!(
                    "row {}, col {}, coord {} ==> {:?} of {:?}",
                    row, col, coord, piece, player
                );
                let field = Some(PlayerPiece { player, piece });
                let index = rowcol2index(row as i32, col as i32);
                board[index] = field;
                col += 1;
            }
        }
    }

    Position::create(board, to_move)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_initial_board_piece_colors() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let pos = parse_fen(fen);

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
                    assert!(pos[&coord].is_some());
                    assert_eq!(player, pos[&coord].unwrap().player);
                }
            }
        }
    }

    #[test]
    fn parse_initial_board_fields() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let pos = parse_fen(fen);

        let white_rook = PlayerPiece::new(Player::White, Piece::Rook);
        let white_king = PlayerPiece::new(Player::White, Piece::King);
        let white_queen = PlayerPiece::new(Player::White, Piece::Queen);

        let black_king = PlayerPiece::new(Player::Black, Piece::King);
        let black_queen = PlayerPiece::new(Player::Black, Piece::Queen);

        assert_eq!(white_rook, pos["A1"].unwrap());
        assert_eq!(white_queen, pos["D1"].unwrap());
        assert_eq!(white_king, pos["E1"].unwrap());

        assert_eq!(black_queen, pos["D8"].unwrap());
        assert_eq!(black_king, pos["E8"].unwrap());
    }

    #[test]
    fn parse_lonely_king() {
        let fen = "7K/8/8/8/8/8/8/8 w KQkq - 0 1";
        let pos = parse_fen(fen);

        let white_king = PlayerPiece::new(Player::White, Piece::King);
        assert_eq!(white_king, pos["H8"].unwrap());
    }
}
