use std::ops::Index;
use std::slice;

#[path="basic_types.rs"]
mod basic_types;

pub use basic_types::*;

pub type Field = Option<PlayerPiece>;
pub type Board = Vec<Field>;

#[derive(Clone, PartialEq, Eq)]
pub struct Position {
    pub to_move: Player,
    pub board: Board,
    can_castle_white: bool,
    can_castle_black: bool,
    pub en_passant: Option<Coord>,
}

impl Position {
    pub fn new() -> Self {
        let board = vec![None; FIELDS_NO];
        let to_move = Player::White;

        Position {
            to_move,
            board,
            can_castle_black: true,
            can_castle_white: true,
            en_passant: None,
        }
    }

    pub fn create(board: Board, to_move: Player, en_passant: Option<Coord>) -> Position {
        Position {
            board,
            to_move,
            can_castle_black: true,
            can_castle_white: true,
            en_passant
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

        let en_passant_str = match self.en_passant {
            None => "-".to_string(),
            Some(coord) => coord.to_ascii_lowercase(),
        };

        // TODO: implement checking castling rights and en passant
        result.push_str(&format!("KQkq {} 0 1", en_passant_str));
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
        match str2coord(i) {
            None =>
                panic!("Wrong coordinate: {}", i),
            Some(coord) => {
                let index = coord2index(coord);
                return &self.board[index];
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_create() {
        let pos = Position::new();

        assert_eq!(pos.to_move, Player::White);
        assert_eq!(pos.en_passant, None);

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