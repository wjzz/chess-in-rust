#[path = "move_gen.rs"]
pub mod move_gen;

pub use move_gen::*;

impl Position {
    pub fn make_move(&mut self, mv: Move) -> Result<(), String> {
        let Move { src, dest, promote_to } = mv;
        let color = self.to_move;

        // verify there is a piece to move at src
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

        let RowCol { row: src_row, col: src_col } = coord2rowcol(src);
        let RowCol { row: dest_row, col: dest_col } = coord2rowcol(dest);
        let en_passant_flag = if piece == Piece::Pawn {

            // initial pawn move
            if (dest_row - src_row).abs() == 2 && src_col == dest_col {
                let ep_row = (src_row + dest_row) / 2;
                Some(rowcol2coord(ep_row, src_col))
            } else {
                None
            }
        } else {
            None
        };

        // TODO: check if we make a capture?

        let new_piece = match promote_to {
            None => piece,
            Some(piece) => piece,
        };

        let prev_en_passant_flag = self.en_passant;

        // if we take en passant, we have to clear another square
        if piece == Piece::Pawn && prev_en_passant_flag.is_some() {
            let ep_dest = prev_en_passant_flag.unwrap();
            if dest == ep_dest {

                let clear_row = src_row;
                let clear_col = dest_col;
                let clear_coord = rowcol2coord_safe(clear_row, clear_col).unwrap();

                self.board[coord2index(clear_coord)] = None;
            }
        }

        // make the actual changes
        self.board[coord2index(src)] = None;
        self.board[coord2index(dest)] = Some(PlayerPiece::new(color, new_piece));
        self.to_move = color.opposite();
        self.en_passant = en_passant_flag;

        Ok(())
    }

    fn perft_immutable_iter(depth: u32, level: u32, pos: Position) -> u32 {
        if depth == 0 {
            return 1;
        }

        let moves = pos.legal_moves();
        let mut result = 0;

        if depth == 1 {
            return moves.len() as u32;
        }

        // if level == 0 && depth > 4 {
        //     println!("Total {} top moves.", moves.len());
        // }

        // let mut counter = 1;
        for &mv in moves.iter() {
            // if level == 0 && depth > 4 {
            //     println!(" {:2}/{}\t{}->{}", counter, moves.len(), mv.src, mv.dest);
            //     counter += 1;
            // }
            let mut pos2 = pos.clone();
            pos2.make_move(mv).unwrap();
            result += Position::perft_immutable_iter(depth-1, level+1, pos2);
        }

        result
    }

    pub fn perft_immutable(depth: u32, fen: &str) -> u32 {
        let pos = Position::from_fen(fen);
        Position::perft_immutable_iter(depth, 0, pos)
    }

    pub fn perft_immutable_par(depth: u32, fen: &str) -> u32 {
        let pos = Position::from_fen(fen);

        let moves = pos.legal_moves();

        use std::sync::Mutex;
        use std::sync::Arc;
        use std::thread;

        let mtx = Arc::new(Mutex::new(0));
        let mut threads = vec![];

        for (counter, mv) in moves.iter().enumerate() {
            if depth > 5 {
                println!(" {:2}/{}\t{}->{}", counter, moves.len(), mv.src, mv.dest);
            }

            let mut pos2 = pos.clone();
            pos2.make_move(*mv).unwrap();

            let mtx2 = Arc::clone(&mtx);

            threads.push(thread::spawn(move || {
                let value = Position::perft_immutable_iter(depth-1, 1, pos2);
                let mut result = mtx2.lock().unwrap();
                *result += value;
            }));
        }


        for t in threads {
            t.join().unwrap();
        }

        let x = *mtx.lock().unwrap(); x
    }
}