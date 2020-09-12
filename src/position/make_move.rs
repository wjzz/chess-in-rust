#[path = "move_gen.rs"]
pub mod move_gen;

pub use move_gen::*;

impl Position {
    pub fn make_move(&mut self, mv: Move) -> Result<(), String> {
        let Move { src, dest, promote_to } = mv;
        let color = self.to_move;

        // verify there is a piece at src
        let piece = match self[src] {
            None =>
                return Err(format!("Expected to find a piece at {}!", src)),
            Some(player_piece) if player_piece.player != color =>
                return Err(format!("Expected to find {}'s piece at {}, but opponent piece found!", color.to_ascii(), src)),
            Some(player_piece) => {
                player_piece.piece
            },
        };

        // TODO: should we check that `piece` can really move to dest (e.g. is this is diagonal move)

        if self[dest].is_some() && self[dest].unwrap().player == color {
            return Err(format!("Can't capture own piece at {}", dest));
        }

        // TODO: check if we make a capture?

        let new_piece = match promote_to {
            None => piece,
            Some(piece) => piece,
        };

        // make the actual changes
        self.board[coord2index(src)] = None;
        self.board[coord2index(dest)] = Some(PlayerPiece::new(color, new_piece));
        self.to_move = color.opposite();

        Ok(())
    }

    fn perft_immutable_iter(depth: u32, pos: Position) -> u32 {
        if depth == 0 {
            return 1;
        }

        let moves = pos.moves();
        let mut result = 0;

        if depth == 1 {
            return moves.len() as u32;
        }

        for &mv in moves.iter() {
            let mut pos2 = pos.clone();
            pos2.make_move(mv).unwrap();
            result += Position::perft_immutable_iter(depth-1, pos2);
        }

        result
    }

    pub fn perft_immutable(depth: u32, fen: &str) -> u32 {
        let pos = Position::from_fen(fen);
        Position::perft_immutable_iter(depth, pos)
    }
}