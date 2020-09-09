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

impl Piece {
    pub fn to_ascii(&self) -> String {
        match *self {
            Piece::Pawn => "p",
            Piece::Knight => "n",
            Piece::Bishop => "b",
            Piece::Rook => "r",
            Piece::Queen => "q",
            Piece::King => "k",
        }
        .to_string()
    }
}

pub type Coord = &'static str;

pub const COORDS: [Coord; FIELDS_NO] = [
    "A1", "A2", "A3", "A4", "A5", "A6", "A7", "A8", "B1", "B2", "B3", "B4", "B5", "B6", "B7", "B8",
    "C1", "C2", "C3", "C4", "C5", "C6", "C7", "C8", "D1", "D2", "D3", "D4", "D5", "D6", "D7", "D8",
    "E1", "E2", "E3", "E4", "E5", "E6", "E7", "E8", "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8",
    "G1", "G2", "G3", "G4", "G5", "G6", "G7", "G8", "H1", "H2", "H3", "H4", "H5", "H6", "H7", "H8",
];

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct RowCol {
    pub row: u32,
    pub col: u32,
}

impl RowCol {
    pub fn new(row: u32, col: u32) -> Self {
        RowCol { row, col }
    }
}

pub fn rowcol2index(row: u32, col: u32) -> usize {
    (row * 8 + col) as usize
}

pub fn rowcol2coord(row: u32, col: u32) -> Coord {
    // NOTE: we swap col and row here intentionally
    let index = rowcol2index(col, row);
    COORDS[index]
}

pub fn coord2rowcol(coord: Coord) -> RowCol {
    let col = match coord.chars().nth(0).unwrap() {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'D' => 3,
        'E' => 4,
        'F' => 5,
        'G' => 6,
        'H' => 7,
        _ => panic!("Wrong index! {}", coord),
    };

    let row = coord.chars().nth(1).unwrap().to_digit(10).unwrap() - 1;
    assert!(row < 8);
    RowCol { row, col }
}

pub fn coord2index(coord: Coord) -> usize {
    let RowCol { row, col } = coord2rowcol(coord);
    rowcol2index(row, col)
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct PlayerPiece {
    pub player: Player,
    pub piece: Piece,
}

impl PlayerPiece {
    pub fn new(player: Player, piece: Piece) -> Self {
        PlayerPiece { player, piece }
    }

    pub fn to_ascii(&self) -> String {
        let piece_ascii = self.piece.to_ascii();
        match self.player {
            Player::Black => piece_ascii,
            Player::White => piece_ascii.to_ascii_uppercase(),
        }
    }
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

    pub fn to_ascii(&self) -> String {
        let mut result = String::new();
        for row in (0..8).rev() {
            for col in 0..8 {
                let ch = match self.board[rowcol2index(row, col)] {
                    None => ".".to_string(),
                    Some(player_piece) => player_piece.to_ascii(),
                };
                result.push_str(&ch);
            }
            result.push('\n');
        }

        result
    }

    pub fn count_pieces_by_player(&self, player: Player) -> usize {
        self.board
            .iter()
            .filter(|f| f.is_some() && f.unwrap().player == player)
            .count()
    }
}

impl Index<&str> for Position {
    type Output = Field;

    fn index(&self, i: &str) -> &Field {
        for &coord in COORDS.iter() {
            if i == coord {
                let index = coord2index(coord);
                return &self.board[index];
            }
        }
        panic!("Wrong coordinate: {}", i);
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

    #[test]
    fn test_coord_conversion_rowcol2coord() {
        assert_eq!("A1", rowcol2coord(0, 0));
        assert_eq!("A8", rowcol2coord(7, 0));
        assert_eq!("H1", rowcol2coord(0, 7));
        assert_eq!("H8", rowcol2coord(7, 7));

        assert_eq!("E4", rowcol2coord(3, 4));
    }

    #[test]
    fn test_coord_conversion_coord2rowcol() {
        assert_eq!(coord2rowcol("A1"), RowCol::new(0, 0));
        assert_eq!(coord2rowcol("A8"), RowCol::new(7, 0));
        assert_eq!(coord2rowcol("H1"), RowCol::new(0, 7));
        assert_eq!(coord2rowcol("H8"), RowCol::new(7, 7));

        assert_eq!(coord2rowcol("E4"), RowCol::new(3, 4));
    }

    #[test]
    fn test_coord_rowcol_and_back() {
        for &coord in COORDS.iter() {
            let RowCol { row, col } = coord2rowcol(coord);
            let coord2 = rowcol2coord(row, col);
            assert_eq!(coord, coord2);
        }
    }
}
