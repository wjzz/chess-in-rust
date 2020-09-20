#[path = "move_gen.rs"]
pub mod move_gen;

pub use move_gen::*;

impl Position {
    pub fn make_move(&mut self, mv: Move) -> Result<(), String> {
        let Move {
            src,
            dest,
            promote_to,
        } = mv;

        let color = self.to_move;
        let opponent = color.opposite();

        // verify there is a piece to move at src
        let piece = match self.board[src] {
            None => {
                println!("FEN = {}", self.to_fen());
                return Err(format!("Expected to find a piece at {}!", index2coord(src)));
            },
            Some(player_piece) if player_piece.player != color => {

                return Err(format!(
                    "Expected to find {}'s piece at {}, but opponent piece found!",
                    color.to_ascii(),
                    src
                ))
            }
            Some(player_piece) => player_piece.piece,
        };

        // TODO: should we check that `piece` can really move to dest (e.g. is this is diagonal move)

        if self.board[dest].is_some() && self.board[dest].unwrap().player == color {
            return Err(format!("Can't capture own piece at {}", dest));
        }

        let RowCol {
            row: src_row,
            col: src_col,
        } = index2rowcol(src);
        let RowCol {
            row: dest_row,
            col: dest_col,
        } = index2rowcol(dest);
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

        self.castling_stack.push(self.castle_rights.clone());
        self.captures.push(self.board[dest]);

        // TODO: check with castling rights

        if self.board[dest].is_some() {
            let dest = index2coord(dest);

            let to_remove = if opponent == Player::White {
                if dest == "A1" {
                    "Q"
                } else if dest == "H1" {
                    "K"
                } else {
                    ""
                }
            } else {
                if dest == "A8" {
                    "q"
                } else if dest == "H8" {
                    "k"
                } else {
                    ""
                }
            };
            let castle_rights_new = self
                .castle_rights
                .chars()
                .filter(|c| !to_remove.contains(*c))
                .collect();
            self.castle_rights = if castle_rights_new != "" {
                castle_rights_new
            } else {
                "-".to_string()
            };
        }

        let new_piece = match promote_to {
            None => piece,
            Some(piece) => piece,
        };


        self.ep_stack.push(self.en_passant);
        let prev_en_passant_flag = self.en_passant;

        // if we take en passant, we have to clear another square
        if piece == Piece::Pawn && prev_en_passant_flag.is_some() {
            let ep_dest = prev_en_passant_flag.unwrap();
            if dest == coord2index(ep_dest) {
                let clear_row = src_row;
                let clear_col = dest_col;
                let clear_coord = rowcol2coord_safe(clear_row, clear_col).unwrap();

                self.board[coord2index(clear_coord)] = None;
            }
        }

        self.half_moves_stack.push(self.half_moves);
        if piece == Piece::Pawn || self.board[dest].is_some() {
            self.half_moves = 0;
        } else {
            self.half_moves += 1;
        }

        // check if we have to take away castling rights
        if piece == Piece::King || piece == Piece::Rook {
            let src = index2coord(src);

            let rights = match (color, piece) {
                (Player::White, Piece::King) => "KQ",
                (Player::Black, Piece::King) => "kq",
                (Player::White, Piece::Rook) if src == "A1" => "Q",
                (Player::White, Piece::Rook) if src == "H1" => "K",
                (Player::Black, Piece::Rook) if src == "A8" => "q",
                (Player::Black, Piece::Rook) if src == "H8" => "k",
                _ => "",
            };
            let castle_rights_new = self
                .castle_rights
                .chars()
                .filter(|c| !rights.contains(*c))
                .collect();
            self.castle_rights = if castle_rights_new != "" {
                castle_rights_new
            } else {
                "-".to_string()
            };
        }

        // check if move is castling
        if piece == Piece::King && self.is_castling_move(mv) {
            let (rook_src, rook_dest) = self.rook_position_castling(mv);
            let rook_piece = PlayerPiece {
                player: color,
                piece: Piece::Rook,
            };
            assert_eq!(Some(rook_piece), self.board[coord2index(rook_src)]);
            self.board[coord2index(rook_src)] = None;
            self.board[coord2index(rook_dest)] = Some(rook_piece);
        }

        // make the actual changes
        self.board[src] = None;
        self.board[dest] = Some(PlayerPiece::new(color, new_piece));
        self.to_move = color.opposite();
        self.en_passant = en_passant_flag;

        if color == Player::Black {
            self.full_moves += 1;
        }

        Ok(())
    }
}

impl Position {
    pub fn unmake_move(&mut self, mv: Move) -> Result<(), String> {
        let Move {
            src,
            dest,
            promote_to,
        } = mv;

        // println!(">> {} ==> {}", index2coord(src), index2coord(dest));

        let color = self.to_move.opposite();
        let opponent = self.to_move;

        // verify there is a piece to move at src
        let piece = match self.board[dest] {
            None => {
                println!("FEN = {}", self.to_fen());
                println!("LAST MOVE = {}", mv.to_usi_ascii());
                return Err(format!("Expected to find a piece at {}!", src));
            },
            Some(player_piece) if player_piece.player != color => {
                println!("FEN = {}", self.to_fen());
                return Err(format!(
                    "Expected to find {}'s piece at {}, but opponent piece found!",
                    color.to_ascii(),
                    dest
                ))
            }
            Some(player_piece) => player_piece.piece,
        };

        // println!(">> piece = {}", piece.to_ascii());

        self.en_passant = self.ep_stack.pop().unwrap();
        self.castle_rights = self.castling_stack.pop().unwrap();

        // Make the actual changes

        let ep_taken_pawn_src = match self.en_passant {
            None => None,
            Some(ep_coord) => {
                // println!(">> EP COORD = {}", ep_coord);
                let ep_field = coord2index(ep_coord);
                // we just made an en passant capture
                if ep_field == dest && piece == Piece::Pawn {
                    let src_rc = index2rowcol(src);
                    let dest_rc = index2rowcol(dest);
                    let captured_pawn_col = dest_rc.col;
                    let captured_pawn_row = src_rc.row;
                    Some(rowcol2index(captured_pawn_row, captured_pawn_col))
                } else {
                    None
                }
            }
        };

        // check if move was castling and unmake the rook move
        if piece == Piece::King && self.is_castling_move_color(mv, color) {
            let (rook_src, rook_dest) = self.rook_position_castling_color(mv, color);
            let rook_piece = PlayerPiece {
                player: color,
                piece: Piece::Rook,
            };
            assert_eq!(Some(rook_piece), self[rook_dest]);
            self.board[coord2index(rook_src)] = Some(rook_piece);
            self.board[coord2index(rook_dest)] = None;
        }

        // unpromote the pawn if needed
        let original_piece = if promote_to.is_some() {
            Some(PlayerPiece { player: color, piece: Piece::Pawn })
        } else {
            self.board[dest]
        };

        match ep_taken_pawn_src {
            None => {
                self.board[dest] = self.captures.pop().unwrap();
            }
            Some(taken_pawn_src) => {
                self.board[dest] = None;
                self.captures.pop().unwrap();
                self.board[taken_pawn_src] = Some(PlayerPiece { piece: Piece::Pawn, player: opponent });
            }
        }

        self.board[src] = original_piece;

        if color == Player::Black {
            self.full_moves -= 1;
        }

        self.half_moves = self.half_moves_stack.pop().unwrap();

        self.to_move = color;

        Ok(())
    }
}