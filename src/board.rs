use std::ops::Index;
use std::slice;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Player {
    White,
    Black,
}

pub const PLAYERS: [Player; 2] = [Player::White, Player::Black];

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub const PIECES: [Piece; 6] = [
    Piece::Pawn,
    Piece::Knight,
    Piece::Bishop,
    Piece::Rook,
    Piece::Queen,
    Piece::King,
];

pub type Coord = &'static str;

pub const COORDS: [Coord; FIELDS_NO] = [
    "A1", "A2", "A3", "A4", "A5", "A6", "A7", "A8", "B1", "B2", "B3", "B4", "B5", "B6", "B7", "B8",
    "C1", "C2", "C3", "C4", "C5", "C6", "C7", "C8", "D1", "D2", "D3", "D4", "D5", "D6", "D7", "D8",
    "E1", "E2", "E3", "E4", "E5", "E6", "E7", "E8", "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8",
    "G1", "G2", "G3", "G4", "G5", "G6", "G7", "G8", "H1", "H2", "H3", "H4", "H5", "H6", "H7", "H8",
];

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct PlayerPiece {
    pub player: Player,
    pub piece: Piece,
}

pub type Field = Option<PlayerPiece>;
pub type Board = Vec<Field>;

pub struct Position {
    to_move: Player,
    board: Board,
    can_castle_white: bool,
    can_castle_black: bool,
    /* what about en passant? */
}

pub const FIELDS_NO: usize = 64;

impl Position {
    pub fn new() -> Self {
        let board = vec![None; FIELDS_NO];
        let to_move = Player::White;

        Position {
            to_move,
            board,
            can_castle_black: true,
            can_castle_white: true,
        }
    }

    pub fn create(board: Board, to_move: Player) -> Position {
        Position {
            board,
            to_move,
            can_castle_black: true,
            can_castle_white: true,
        }
    }

    pub fn fields(&self) -> slice::Iter<Field> {
        self.board.iter()
    }

    pub fn count_pieces_by_player(&self, player: Player) -> usize {
        self.board
            .iter()
            .filter(|f| f.is_some() && f.unwrap().player == player)
            // .filter(|f| f.map_or(false, |pp| pp.player == player))
            .count()
    }
}

impl Index<&str> for Position {
    type Output = Field;

    fn index(&self, i: &str) -> &Field {
        let col = match i.chars().nth(0).unwrap() {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            'D' => 3,
            'E' => 4,
            'F' => 5,
            'G' => 6,
            'H' => 7,
            _ => panic!("Wrong index! {}", i),
        };

        let row = i.chars().nth(1).unwrap().to_digit(10).unwrap() - 1;
        assert!(row < 8);

        let index = col * 8 + row as usize;
        // replace manual calc with fn
        &self.board[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_create() {
        let pos = Position::new();

        assert_eq!(pos.to_move, Player::White);

        for &field in pos.fields() {
            assert_eq!(field, None);
        }
    }

    #[test]
    fn count_pieces_new() {
        let pos = Position::new();

        for &player in PLAYERS.iter() {
            assert_eq!(0, pos.count_pieces_by_player(player));
        }
    }

    #[test]
    fn check_indexing_new() {
        let pos = Position::new();

        for coord in COORDS.iter() {
            assert_eq!(None, pos[coord]);
        }
    }
}
