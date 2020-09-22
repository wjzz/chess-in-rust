use std::ops::Index;
use std::slice;

#[path = "basic_types.rs"]
mod basic_types;

pub use basic_types::*;

pub type Field = Option<PlayerPiece>;
// pub type Board = Vec<Field>;
pub type Board = Vec<BoardCell>;

#[derive(Clone, PartialEq, Eq)]
pub struct Position {
    pub board: Board,
    pub to_move: Player,
    pub castle_rights: String,
    pub en_passant: Option<usize>,
    pub half_moves: u32, // TODO: we could remove this and only leave the stack
    pub full_moves: u32,
    pub half_moves_stack: Vec<u32>,
    pub captures: Vec<BoardCell>,
    pub ep_stack: Vec<Option<usize>>,
    pub castling_stack: Vec<String>,
    pub kings: [usize; 2],
}

impl Position {
    pub fn create(
        board: Board,
        to_move: Player,
        en_passant: Option<usize>,
        castle_rights: String,
        half_moves: u32,
        full_moves: u32,
        kings: [usize; 2]
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
            kings,
        }
    }

    pub fn fields(&self) -> slice::Iter<BoardCell> {
        self.board.iter()
    }

    pub fn to_ascii(&self) -> String {
        let mut result = String::new();
        for row in (0..8).rev() {
            for col in 0..8 {
                let ch = boardcell_to_ascii(self.board[rowcol2index(row, col)]);
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
                    EMPTY => empty_in_a_row += 1,
                    player_piece => {
                        if empty_in_a_row > 0 {
                            line.push_str(&empty_in_a_row.to_string());
                            empty_in_a_row = 0;
                        }
                        line.push_str(&boardcell_to_ascii(player_piece));
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
            Some(field) => index2coord(field).to_ascii_lowercase(),
        };

        result.push_str(&format!(
            "{} {} {} {}",
            self.castle_rights, en_passant_str, self.half_moves, self.full_moves
        ));
        result
    }
}

impl Index<&str> for Position {
    type Output = BoardCell;

    fn index(&self, i: &str) -> &BoardCell {
        match str2coord(i) {
            None => panic!("Wrong coordinate: {}", i),
            Some(coord) => {
                let index = coord2index(coord);
                return &self.board[index];
            }
        }
    }
}