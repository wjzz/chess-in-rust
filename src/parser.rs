#[path = "board.rs"]
mod board;

use board::{Piece, Player, PlayerPiece, Position, FIELDS_NO};

pub fn parseFEN(fen: &str) -> Position {
    let mut board = vec![None; FIELDS_NO];

    let to_move = Player::White;

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

    for (col, line) in board_str.split('/').rev().enumerate() {
        for ch in line.chars() {
            let mut row = 7;
            if ch.is_ascii_digit() {
                let val = ch.to_digit(10).unwrap();
                row -= val-1;
            } else {
                let player = if ch.is_uppercase() { Player::White } else { Player::Black };
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
                let field = Some(PlayerPiece { player, piece });
                board[col * 8 + row as usize] = field;
            }
        }
    }

    Position::create(board, to_move)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_initial_board() {
        let FEN = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

        let pos = parseFEN(FEN);

        let piece_a1 = pos["A1"];
        assert!(piece_a1.is_some());
        assert_eq!(piece_a1.unwrap().player, Player::White);
    }
}
