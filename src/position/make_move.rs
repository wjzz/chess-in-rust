#[path = "parser.rs"]
pub mod parser;

pub use parser::*;

impl Position {
    pub fn make_move(&mut self, mv: IntMove) -> Result<(), String> {
        let (src, dest, promote_to) = intmove_destructure(mv);

        let color = self.to_move;
        let opponent = color.opposite();

        // verify there is a piece to move at src
        let boardpiece = self.board[src];
        let piece = match boardpiece {
            EMPTY => {
                println!("{}", self.to_ascii());
                println!("FEN = {}", self.to_fen());
                return Err(format!("Expected to find a piece at {}!", index2coord(src)));
            },
            player_piece if boardcell_player(player_piece) != color => {

                return Err(format!(
                    "Expected to find {}'s piece at {}, but opponent piece found!",
                    color.to_ascii(),
                    src
                ))
            }
            player_piece => boardcell_piece(player_piece),
        };

        // TODO: should we check that `piece` can really move to dest (e.g. is this is diagonal move)

        if self.board[dest] != EMPTY && boardcell_player(self.board[dest]) == color {
            return Err(format!("Can't capture own piece at {}", dest));
        }

        // update the undo stack
        self.hash_stack.push(self.hash);
        self.castling_stack.push(self.castle_rights.clone());
        self.captures.push(self.board[dest]);
        self.ep_stack.push(self.en_passant);
        let prev_en_passant_flag = self.en_passant;

        // update king locations
        if piece == Piece::King {
            self.kings[color as usize] = dest;
        }

        // set the en_passant_flag only if there is a pawn that could actually take it
        let mut en_passant_flag = None;
        if piece == Piece::Pawn && dest.max(src) - dest.min(src) == 32 {
            for delta in [-1, 1].iter() {
                let pawn_field = dest as i32 + delta;
                if (pawn_field & 0x88) == 0 && self.board[pawn_field as usize] == -boardpiece {
                    en_passant_flag = Some((dest + src) / 2);
                    break;
                }
            }
        }

        if self.board[dest] != EMPTY {
            unsafe {
                self.hash ^= HASH_BOARD[(self.board[dest] + 6) as usize][dest];
            }

            let dest = index2coord(dest);
            let opp = opponent as usize;

            if (opp == WHITE && dest == "A1") || (opp == BLACK && dest == "A8") {
                if self.castle_rights[opp][QUEENSIDE] {
                    unsafe {
                        self.hash ^= HASH_CASTLING[opp][QUEENSIDE];
                    }
                }
                self.castle_rights[opp][QUEENSIDE] = false;
            }
            if (opp == WHITE && dest == "H1") || (opp == BLACK && dest == "H8") {
                if self.castle_rights[opp][KINGSIDE] {
                    unsafe {
                        self.hash ^= HASH_CASTLING[opp][KINGSIDE];
                    }
                }
                self.castle_rights[opp][KINGSIDE] = false;
            }
        }

        let new_piece = match promote_to {
            None => piece,
            Some(piece) => piece,
        };

        // if we take en passant, we have to clear another square
        if piece == Piece::Pawn && prev_en_passant_flag.is_some() {
            let ep_dest = prev_en_passant_flag.unwrap();
            if dest == ep_dest {
                let clear_row = index2rowcol(src).row;
                let clear_col = index2rowcol(dest).col;

                let opp_pawn_index = rowcol2index(clear_row, clear_col);
                let opp_pawn = self.board[opp_pawn_index];

                self.board[opp_pawn_index] = EMPTY;

                unsafe {
                    self.hash ^= HASH_BOARD[(opp_pawn + 6) as usize][opp_pawn_index];
                }
            }
        }

        self.half_moves_stack.push(self.half_moves);
        if piece == Piece::Pawn || self.board[dest] != EMPTY {
            self.half_moves = 0;
        } else {
            self.half_moves += 1;
        }

        // check if we have to take away castling rights
        if piece == Piece::King || piece == Piece::Rook {
            let src = index2coord(src);

            let col = color as usize;

            if piece == Piece::King {
                unsafe {
                    if self.castle_rights[col][KINGSIDE] {
                        self.hash ^= HASH_CASTLING[col][KINGSIDE];
                    }
                    if self.castle_rights[col][QUEENSIDE] {
                        self.hash ^= HASH_CASTLING[col][QUEENSIDE];
                    }
                }

                self.castle_rights[col][KINGSIDE] = false;
                self.castle_rights[col][QUEENSIDE] = false;
            } else {
                // rook

                if (col == WHITE && src == "A1") || (col == BLACK && src == "A8") {
                    unsafe {
                        if self.castle_rights[col][QUEENSIDE] {
                            self.hash ^= HASH_CASTLING[col][QUEENSIDE];
                        }
                    }
                    self.castle_rights[col][QUEENSIDE] = false;
                }
                if (col == WHITE && src == "H1") || (col == BLACK && src == "H8") {
                    unsafe {
                        if self.castle_rights[col][KINGSIDE] {
                            self.hash ^= HASH_CASTLING[col][KINGSIDE];
                        }
                    }
                    self.castle_rights[col][KINGSIDE] = false;
                }
            }
        }

        // check if move is castling
        if intmove_is_castle(mv) || (piece == Piece::King && (src as i32 - dest as i32).abs() == 2) {
            // TODO: remove the coords here
            let (rook_src_coord, rook_dest_coord) = self.rook_position_castling(mv);

            let rook_src = coord2index(rook_src_coord);
            let rook_dest = coord2index(rook_dest_coord);

            let rook_piece = boardcell_encode(color, Piece::Rook);
            assert_eq!(rook_piece, self.board[rook_src]);

            self.castle_rights[color as usize][KINGSIDE] = false;
            self.castle_rights[color as usize][QUEENSIDE] = false;

            self.board[rook_src] = EMPTY;
            self.board[rook_dest] = rook_piece;

            unsafe {
                self.hash ^= HASH_CASTLING[color as usize][KINGSIDE];
                self.hash ^= HASH_CASTLING[color as usize][QUEENSIDE];

                self.hash ^= HASH_BOARD[(rook_piece + 6) as usize][rook_src];
                self.hash ^= HASH_BOARD[(rook_piece + 6) as usize][rook_dest];
            }
        }

        // make the actual changes
        let new_boardpiece = boardcell_encode(color, new_piece);
        self.board[src] = EMPTY;
        self.board[dest] = new_boardpiece;
        self.to_move = color.opposite();
        self.en_passant = en_passant_flag;

        if color == Player::Black {
            self.full_moves += 1;
        }

        unsafe {
            self.hash ^= HASH_BOARD[(boardpiece + 6) as usize][src];
            self.hash ^= HASH_BOARD[(new_boardpiece + 6) as usize][dest];

            self.hash ^= HASH_TO_MOVE[color as usize];
            self.hash ^= HASH_TO_MOVE[1 - (color as usize)];

            self.hash ^= Position::en_passant_to_hash(prev_en_passant_flag);
            self.hash ^= Position::en_passant_to_hash(en_passant_flag);
        }


        Ok(())
    }
}

impl Position {
    pub fn unmake_move(&mut self, mv: IntMove) -> Result<(), String> {
        let (src, dest, promote_to) = intmove_destructure(mv);

        // println!(">> {} ==> {}", index2coord(src), index2coord(dest));

        let color = self.to_move.opposite();
        let opponent = self.to_move;

        // verify there is a piece to move at src
        let piece = match self.board[dest] {
            EMPTY => {
                println!("FEN = {}", self.to_fen());
                // println!("LAST MOVE = {}", mv.to_usi_ascii());
                return Err(format!("Expected to find a piece at {}!", src));
            },
            player_piece if boardcell_player(player_piece) != color => {
                println!("FEN = {}", self.to_fen());
                return Err(format!(
                    "Expected to find {}'s piece at {}, but opponent piece found!",
                    color.to_ascii(),
                    dest
                ))
            }
            player_piece => boardcell_piece(player_piece),
        };

        if piece == Piece::King {
            self.kings[color as usize] = src;
        }

        // println!(">> piece = {}", piece.to_ascii());

        self.en_passant = self.ep_stack.pop().unwrap();
        self.castle_rights = self.castling_stack.pop().unwrap();

        // Make the actual changes

        let ep_taken_pawn_src = match self.en_passant {
            None => None,
            Some(ep_coord) => {
                // println!(">> EP COORD = {}", ep_coord);
                let ep_field = ep_coord;
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
        if intmove_is_castle(mv) || (piece == Piece::King && (src as i32 - dest as i32).abs() == 2){
            let (rook_src, rook_dest) = self.rook_position_castling_color(mv, color);
            let rook_piece = boardcell_encode(color, Piece::Rook);
            assert_eq!(rook_piece, self[rook_dest]);

            self.board[coord2index(rook_src)] = rook_piece;
            self.board[coord2index(rook_dest)] = EMPTY;
        }

        // unpromote the pawn if needed
        let original_piece = if promote_to.is_some() {
            boardcell_encode(color, Piece::Pawn)
        } else {
            self.board[dest]
        };

        match ep_taken_pawn_src {
            None => {
                self.board[dest] = self.captures.pop().unwrap();
            }
            Some(taken_pawn_src) => {
                self.board[dest] = EMPTY;
                self.captures.pop().unwrap();
                self.board[taken_pawn_src] = boardcell_encode(opponent, Piece::Pawn);
            }
        }

        self.board[src] = original_piece;

        if color == Player::Black {
            self.full_moves -= 1;
        }

        self.hash = self.hash_stack.pop().unwrap();

        self.half_moves = self.half_moves_stack.pop().unwrap();

        self.to_move = color;

        Ok(())
    }
}
