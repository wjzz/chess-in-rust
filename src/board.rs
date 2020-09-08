use std::slice;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Player {
    White,
    Black,
}

const PLAYERS: [Player; 2] = [Player::White, Player::Black];

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

const PIECES: [Piece; 6] = [
    Piece::Pawn,
    Piece::Knight,
    Piece::Bishop,
    Piece::Rook,
    Piece::Queen,
    Piece::King,
];

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct PlayerPiece {
    player: Player,
    piece: Piece,
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

const FIELDS_NO: usize = 64;

impl Position {
    fn new() -> Self {
        let board = vec![None; FIELDS_NO];
        let to_move = Player::White;

        Position {
            to_move,
            board,
            can_castle_black: true,
            can_castle_white: true,
        }
    }

    fn fields(&self) -> slice::Iter<Field> {
        self.board.iter()
    }

    fn count_pieces_by_player(&self, player: Player) -> usize {
        self.board
            .iter()
            .filter(|f| f.is_some() && f.unwrap().player == player)
            // .filter(|f| f.map_or(false, |pp| pp.player == player))
            .count()
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
}
