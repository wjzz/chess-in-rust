use std::ops::Index;
use std::slice;

#[path = "basic_types.rs"]
mod basic_types;

pub use basic_types::*;

pub type Field = Option<PlayerPiece>;
// pub type Board = Vec<Field>;
pub type Board = Vec<BoardCell>;

pub type CastleRights = [[bool;2];2];

pub const WHITE: usize = 0;
pub const BLACK: usize = 1;

pub const KINGSIDE: usize = 0;
pub const QUEENSIDE: usize = 1;

#[derive(Clone, PartialEq, Eq)]
pub struct Position {
    pub board: Board,
    pub to_move: Player,
    pub castle_rights: CastleRights,
    pub en_passant: Option<usize>,
    pub half_moves: u32, // TODO: we could remove this and only leave the stack
    pub full_moves: u32,
    pub half_moves_stack: Vec<u32>,
    pub captures: Vec<BoardCell>,
    pub ep_stack: Vec<Option<usize>>,
    pub castling_stack: Vec<CastleRights>,
    pub kings: [usize; 2],
    pub hash: u64,
}

pub static mut HASH_INITIALIZED: bool = false;
pub static mut HASH_BOARD: [[u64; 128]; 13] = [[0; 128]; 13];
pub static mut HASH_TO_MOVE: [u64; 2] = [0; 2];
pub static mut HASH_EN_PASSANT: [u64; 128] = [0; 128];

impl Position {
    fn initial_hash(board: &Board, to_move: Player, castle_rights: &CastleRights, en_passant: Option<usize>) -> u64 {
        unsafe {
            if !HASH_INITIALIZED {
                panic!("HASH NOT INITIALIZED");
            }
            let mut h = 0;
            for &i in INDEXES88.iter() {
                if board[i] != EMPTY {
                    h ^= HASH_BOARD[i][(board[i] + 6) as usize];
                }
            }
            h ^= HASH_TO_MOVE[to_move as usize];

            // TODO: change castling rights into a more efficient repr

            match en_passant {
                None => h ^= HASH_EN_PASSANT[0],
                Some(ep_field) => h ^= HASH_EN_PASSANT[ep_field],
            }
            h
        }
    }

    pub fn create(
        board: Board,
        to_move: Player,
        en_passant: Option<usize>,
        castle_rights: CastleRights,
        half_moves: u32,
        full_moves: u32,
        kings: [usize; 2]
    ) -> Position {
        let hash = 0; //Position::initial_hash(&board, to_move, &castle_rights, en_passant);
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
            hash,
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

        let mut castle_str = String::new();
        if self.castle_rights[0][KINGSIDE] {
            castle_str.push('K');
        }
        if self.castle_rights[0][QUEENSIDE] {
            castle_str.push('Q');
        }
        if self.castle_rights[1][KINGSIDE] {
            castle_str.push('k');
        }
        if self.castle_rights[1][QUEENSIDE] {
            castle_str.push('q');
        }
        if castle_str == "" {
            castle_str = String::from("-");
        }

        result.push_str(&format!(
            "{} {} {} {}",
            castle_str, en_passant_str, self.half_moves, self.full_moves
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