pub const FIELDS_NO: usize = 64;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Player {
    White,
    Black,
}

impl Player {
    pub fn opposite(&self) -> Self {
        match self {
            Player::Black => Player::White,
            Player::White => Player::Black,
        }
    }

    pub fn to_ascii(&self) -> String {
        match self {
            Player::Black => "b",
            Player::White => "w",
        }
        .to_string()
    }
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

pub const PROMOTABLE_PIECES: [Piece; 4] = [Piece::Knight, Piece::Bishop, Piece::Rook, Piece::Queen];

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

pub fn str2coord(s: &str) -> Option<Coord> {
    for &coord in COORDS.iter() {
        if coord == s {
            return Some(coord);
        }
    }
    return None;
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct RowCol {
    pub row: i32,
    pub col: i32,
}

impl RowCol {
    pub fn new(row: i32, col: i32) -> Self {
        RowCol { row, col }
    }
}

pub fn rowcol2index(row: i32, col: i32) -> usize {
    (row * 8 + col) as usize
}

pub fn rowcol2coord(row: i32, col: i32) -> Coord {
    // NOTE: we swap col and row here intentionally
    let index = rowcol2index(col, row);
    COORDS[index]
}

pub fn rowcol2coord_safe(row: i32, col: i32) -> Option<Coord> {
    if 0 > row || row >= 8 || 0 > col || col >= 8 {
        return None;
    }
    Some(rowcol2coord(row, col))
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

    let row = (coord.chars().nth(1).unwrap().to_digit(10).unwrap() - 1) as i32;
    assert!(row < 8);
    RowCol { row, col }
}

pub fn coord2index(coord: Coord) -> usize {
    let RowCol { row, col } = coord2rowcol(coord);
    rowcol2index(row, col)
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Move {
    pub src: Coord,
    pub dest: Coord,
    pub promote_to: Option<Piece>,
}

impl Move {
    pub fn new(src: Coord, dest: Coord, promote_to: Option<Piece>) -> Self {
        Move {
            src,
            dest,
            promote_to,
        }
    }

    pub fn from_ascii(ascii: &'static str) -> Self {
        let src = &ascii[0..2];
        assert_eq!("->", &ascii[2..4]);
        let dest = &ascii[4..6];

        // TODO: ignore promotion for the time being, it could be denoted as E7->E8=Q
        Move::new(src, dest, None)
    }
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

#[cfg(test)]
mod tests {
    use super::*;

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
