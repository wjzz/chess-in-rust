#[path = "basic_types.rs"]
mod basic_types;

use std::ops::Index;
use std::slice;

use rand::thread_rng;

pub use basic_types::*;

pub type Field = Option<PlayerPiece>;
pub type Board = Vec<BoardCell>;

pub const WHITE: usize = 0;
pub const BLACK: usize = 1;

pub type CastleRights = [[bool;2];2];
pub const KINGSIDE: usize = 0;
pub const QUEENSIDE: usize = 1;

pub static mut HASH_INITIALIZED: bool = false;
pub static mut HASH_BOARD: [[u64; 128]; 13] = [[0; 128]; 13];
pub static mut HASH_TO_MOVE: [u64; 2] = [0; 2];
pub static mut HASH_CASTLING: [[u64; 2]; 2] = [[0; 2]; 2];
pub static mut HASH_EN_PASSANT: [u64; 128] = [0; 128];

pub fn initialize_hash() {
    use rand::RngCore;

    let mut rng = thread_rng();

    unsafe {
        for piece in 0..13 {
            for i in 0..128 {
                HASH_BOARD[piece][i] = rng.next_u64();
            }
        }

        for pl in 0..2 {
            HASH_TO_MOVE[pl] = rng.next_u64();
            for side in 0..2 {
                HASH_CASTLING[pl][side] = rng.next_u64();
            }
        }

        for i in 0..128 {
            HASH_EN_PASSANT[i] = rng.next_u64();
        }

        HASH_INITIALIZED = true;
        println!("Hashing tables generated!");
    }
}

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
    pub hash_stack: Vec<u64>,
}

impl Position {
    pub fn en_passant_to_hash(en_passant: Option<usize>) -> u64 {
        unsafe {
            match en_passant {
                None => HASH_EN_PASSANT[0],
                Some(ep_field) => HASH_EN_PASSANT[ep_field],
            }
        }
    }

    fn initial_hash(board: &Board, to_move: Player, castle_rights: &CastleRights, en_passant: Option<usize>) -> u64 {
        unsafe {
            if !HASH_INITIALIZED {
                println!("HASH NOT INITIALIZED");
                initialize_hash();
            }

            let mut h = 0;

            for &i in INDEXES88.iter() {
                if board[i] != EMPTY {
                    h ^= HASH_BOARD[(board[i] + 6) as usize][i];
                }
            }

            h ^= HASH_TO_MOVE[to_move as usize];

            for &player in [WHITE, BLACK].iter() {
                for &direction in [KINGSIDE, QUEENSIDE].iter() {
                    if castle_rights[player][direction] {
                        h ^= HASH_CASTLING[player][direction];
                    }
                }
            }

            h ^= Position::en_passant_to_hash(en_passant);

            // println!("hash = {:10x}", h);

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
        assert_eq!(FIELDS88, board.len());
        let hash = Position::initial_hash(&board, to_move, &castle_rights, en_passant);
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
            hash_stack: vec![],
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