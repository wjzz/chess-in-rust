use std::ops::Index;
use std::slice;

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
        }.to_string()
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
    src: Coord,
    dest: Coord,
    piece: Piece,
    promote_to: Option<Piece>,
}

impl Move {
    pub fn new(src: Coord, dest: Coord, piece: Piece, promote_to: Option<Piece>) -> Self {
        Move {
            src,
            dest,
            piece,
            promote_to,
        }
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

pub type Field = Option<PlayerPiece>;
pub type Board = Vec<Field>;

pub struct Position {
    pub to_move: Player,
    pub board: Board,
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

    pub fn to_fen(&self) -> String {
        let mut lines = vec![];
        for row in (0..8).rev() {
            let mut line = String::new();
            let mut empty_in_a_row = 0;

            for col in 0..8 {
                match self.board[rowcol2index(row, col)] {
                    None => empty_in_a_row += 1,
                    Some(player_piece) => {
                        if empty_in_a_row > 0 {
                            line.push_str(&empty_in_a_row.to_string());
                            empty_in_a_row = 0;
                        }
                        line.push_str(&player_piece.to_ascii());
                    }
                };
            }
            if empty_in_a_row > 0 {
                line.push_str(&empty_in_a_row.to_string());
            }
            lines.push(line);
        }
        let mut result = lines.join("/");
        result.push_str(&format!(" {} ", self.to_move.to_ascii()));

        // TODO: implement checking castling rights and en passant
        result.push_str("KQkq - 0 1");
        result
    }

    pub fn count_pieces_by_player(&self, player: Player) -> usize {
        self.board
            .iter()
            .filter(|f| f.is_some() && f.unwrap().player == player)
            .count()
    }
}

impl Position {
    fn line(&self, src: Coord, dx: i32, dy: i32, all_moves: &mut Vec<Move>) {
        let RowCol { row: src_row, col: src_col } = coord2rowcol(src);

        let mut dest_row = src_row + dx;
        let mut dest_col = src_col + dy;

        let PlayerPiece {
            player: color,
            piece,
        } = self[src].unwrap();
        let opp_color = color.opposite();

        while let Some(dest) = rowcol2coord_safe(dest_row, dest_col) {
            let dest_field = self[dest];

            if dest_field.is_none() || dest_field.unwrap().player == opp_color {
                all_moves.push(Move::new(src, dest, piece, None));

                if dest_field.is_some() {
                    return;
                }
            } else {
                return;
            }

            dest_row += dx;
            dest_col += dy;
        }
    }

    fn try_add(&self, src: Coord, dest_row: i32, dest_col: i32, all_moves: &mut Vec<Move>) {
        if let Some(dest) = rowcol2coord_safe(dest_row, dest_col) {
            let PlayerPiece {
                player: color,
                piece,
            } = self[src].unwrap();
            let opp_color = color.opposite();
            let dest_field = self[dest];

            if dest_field.is_none() || dest_field.unwrap().player == opp_color {
                if piece != Piece::Pawn {
                    all_moves.push(Move::new(src, dest, piece, None));
                } else {
                    let reaches_last_row = match color {
                        Player::White => dest_row == 7,
                        Player::Black => dest_row == 0,
                    };

                    if reaches_last_row {
                        for &promo_piece in PROMOTABLE_PIECES.iter() {
                            all_moves.push(Move::new(src, dest, piece, Some(promo_piece)));
                        }
                    } else {
                        all_moves.push(Move::new(src, dest, piece, None));
                    }
                }
            }
        }
    }

    fn generate_moves_from(&self, src: Coord, piece: Piece, color: Player) -> Vec<Move> {
        assert_eq!(PlayerPiece::new(color, piece), self[src].unwrap());

        let RowCol {
            row: src_row,
            col: src_col,
        } = coord2rowcol(src);

        let row_delta: i32 = if color == Player::White { 1 } else { -1 };

        let mut all_moves = vec![];

        // TODO: implement castling

        match piece {
            Piece::Pawn => {
                // TODO: implement en passant

                let is_first_move = match color {
                    Player::White => src_row == 1,
                    Player::Black => src_row == 6,
                };

                self.try_add(src, src_row + row_delta, src_col, &mut all_moves);

                // first move by two squares
                if is_first_move {
                    self.try_add(src, src_row + row_delta * 2, src_col, &mut all_moves);
                }

                // captures
                for col_delta in [-1, 1].iter() {
                    let dest_row = src_row + row_delta;
                    let dest_col = src_col + col_delta;
                    if let Some(dest) = rowcol2coord_safe(dest_row, dest_col) {
                        let dest_piece = self[dest];
                        if dest_piece.is_some() && dest_piece.unwrap().player != color {
                            self.try_add(src, src_row + row_delta, src_col + col_delta, &mut all_moves);
                        }
                    }
                }
            }
            Piece::King => {
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if dx != 0 || dy != 0 {
                            self.try_add(src, src_row + dy, src_col + dx, &mut all_moves);
                        }
                    }
                }
            },
            Piece::Knight => {
                for (w, d) in [(1,2), (2,1)].iter() {
                    for s1 in [-1, 1].iter() {
                        for s2 in [-1, 1].iter() {
                            let dx = w * s1;
                            let dy = d * s2;
                            self.try_add(src, src_row + dy, src_col + dx, &mut all_moves);
                        }
                    }
                }
            },
            Piece::Queen => {
                self.line(src, 0, -1, &mut all_moves);
                self.line(src, 0, 1, &mut all_moves);
                self.line(src, -1, 0, &mut all_moves);
                self.line(src, 1, 0, &mut all_moves);
                self.line(src, 1, -1, &mut all_moves);
                self.line(src, 1, 1, &mut all_moves);
                self.line(src, -1, -1, &mut all_moves);
                self.line(src, -1, 1, &mut all_moves);
            },
            Piece::Bishop => {
                self.line(src, 1, -1, &mut all_moves);
                self.line(src, 1, 1, &mut all_moves);
                self.line(src, -1, -1, &mut all_moves);
                self.line(src, -1, 1, &mut all_moves);
            },
            Piece::Rook => {
                self.line(src, 0, -1, &mut all_moves);
                self.line(src, 0, 1, &mut all_moves);
                self.line(src, -1, 0, &mut all_moves);
                self.line(src, 1, 0, &mut all_moves);
            },
        }

        all_moves
    }

    pub fn moves(&self) -> Vec<Move> {
        let current_color = self.to_move;

        let mut all_moves = vec![];

        for coord in COORDS.iter() {
            if let Some(player_piece) = self[coord] {
                if player_piece.player == current_color {
                    all_moves.append(&mut self.generate_moves_from(
                        coord,
                        player_piece.piece,
                        current_color,
                    ));
                }
            }
        }

        all_moves
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
