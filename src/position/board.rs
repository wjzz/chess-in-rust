use std::ops::Index;
use std::slice;

#[path = "basic_types.rs"]
mod basic_types;

pub use basic_types::*;

pub type Field = Option<PlayerPiece>;
pub type Board = Vec<Field>;

#[derive(Clone, PartialEq, Eq)]
pub struct Position {
    pub board: Board,
    pub to_move: Player,
    pub castle_rights: String,
    pub en_passant: Option<Coord>,
    pub half_moves: u32, // FIXME: we could remove this and only leave the stack
    pub full_moves: u32,
    pub half_moves_stack: Vec<u32>,
    pub captures: Vec<Field>,
    pub ep_stack: Vec<Option<Coord>>,
    pub castling_stack: Vec<String>,
}

impl Position {
    pub fn new() -> Self {
        let board = vec![None; FIELDS_NO];
        let to_move = Player::White;

        Position {
            to_move,
            board,
            castle_rights: String::from("KQkq"),
            en_passant: None,
            half_moves: 0,
            full_moves: 1,
            half_moves_stack: vec![],
            captures: vec![],
            ep_stack: vec![],
            castling_stack: vec![],
        }
    }

    pub fn create(
        board: Board,
        to_move: Player,
        en_passant: Option<Coord>,
        castle_rights: String,
        half_moves: u32,
        full_moves: u32,
    ) -> Position {
        Position {
            board,
            to_move,
            castle_rights,
            en_passant,
            half_moves,
            full_moves,
            half_moves_stack: vec![],
            captures: vec![],
            ep_stack: vec![],
            castling_stack: vec![],
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

        // TODO: implement checking castling rights
        result.push_str(&format!(
            "{} {} {} {}",
            self.castle_rights, en_passant_str, self.half_moves, self.full_moves
        ));
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
            None => panic!("Wrong coordinate: {}", i),
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
