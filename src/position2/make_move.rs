#[path = "parser.rs"]
pub mod parser;

pub use parser::*;

impl Position {
    pub fn make_move(&mut self, mv: IntMove) -> Result<(), String> {
        let (src, dest, promote_to) = intmove_destructure(mv);

        let color = self.to_move;
        let opponent = color.opposite();

        // verify there is a piece to move at src
        let piece = match self.board[src] {
            EMPTY => {
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
                Some(rowcol2index(ep_row, src_col))
            } else {
                None
            }
        } else {
            None
        };

        self.castling_stack.push(self.castle_rights.clone());
        self.captures.push(self.board[dest]);

        // TODO: check with castling rights

        if self.board[dest] != EMPTY {
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
            if dest == ep_dest {
                let clear_row = src_row;
                let clear_col = dest_col;
                let clear_coord = rowcol2coord_safe(clear_row, clear_col).unwrap();

                self.board[coord2index(clear_coord)] = EMPTY;
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
            let rook_piece = boardcell_encode(color, Piece::Rook);
            assert_eq!(rook_piece, self.board[coord2index(rook_src)]);

            self.board[coord2index(rook_src)] = EMPTY;
            self.board[coord2index(rook_dest)] = rook_piece;
        }

        // make the actual changes
        self.board[src] = EMPTY;
        self.board[dest] = boardcell_encode(color, new_piece);
        self.to_move = color.opposite();
        self.en_passant = en_passant_flag;

        if color == Player::Black {
            self.full_moves += 1;
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
        if piece == Piece::King && self.is_castling_move_color(mv, color) {
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

        self.half_moves = self.half_moves_stack.pop().unwrap();

        self.to_move = color;

        Ok(())
    }
}

#[cfg(test)]
mod test_moves {

    use super::*;

    #[test]
    fn make_move_starting() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

        let inputs = [
            // pawns (2 squares)
            (
                "A2->A4",
                "rnbqkbnr/pppppppp/8/8/P7/8/1PPPPPPP/RNBQKBNR b KQkq a3 0 1",
            ),
            (
                "B2->B4",
                "rnbqkbnr/pppppppp/8/8/1P6/8/P1PPPPPP/RNBQKBNR b KQkq b3 0 1",
            ),
            (
                "C2->C4",
                "rnbqkbnr/pppppppp/8/8/2P5/8/PP1PPPPP/RNBQKBNR b KQkq c3 0 1",
            ),
            (
                "D2->D4",
                "rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq d3 0 1",
            ),
            (
                "E2->E4",
                "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
            ),
            (
                "F2->F4",
                "rnbqkbnr/pppppppp/8/8/5P2/8/PPPPP1PP/RNBQKBNR b KQkq f3 0 1",
            ),
            (
                "G2->G4",
                "rnbqkbnr/pppppppp/8/8/6P1/8/PPPPPP1P/RNBQKBNR b KQkq g3 0 1",
            ),
            (
                "H2->H4",
                "rnbqkbnr/pppppppp/8/8/7P/8/PPPPPPP1/RNBQKBNR b KQkq h3 0 1",
            ),
            // pawns (1 square)
            (
                "A2->A3",
                "rnbqkbnr/pppppppp/8/8/8/P7/1PPPPPPP/RNBQKBNR b KQkq - 0 1",
            ),
            (
                "B2->B3",
                "rnbqkbnr/pppppppp/8/8/8/1P6/P1PPPPPP/RNBQKBNR b KQkq - 0 1",
            ),
            (
                "C2->C3",
                "rnbqkbnr/pppppppp/8/8/8/2P5/PP1PPPPP/RNBQKBNR b KQkq - 0 1",
            ),
            (
                "D2->D3",
                "rnbqkbnr/pppppppp/8/8/8/3P4/PPP1PPPP/RNBQKBNR b KQkq - 0 1",
            ),
            (
                "E2->E3",
                "rnbqkbnr/pppppppp/8/8/8/4P3/PPPP1PPP/RNBQKBNR b KQkq - 0 1",
            ),
            (
                "F2->F3",
                "rnbqkbnr/pppppppp/8/8/8/5P2/PPPPP1PP/RNBQKBNR b KQkq - 0 1",
            ),
            (
                "G2->G3",
                "rnbqkbnr/pppppppp/8/8/8/6P1/PPPPPP1P/RNBQKBNR b KQkq - 0 1",
            ),
            (
                "H2->H3",
                "rnbqkbnr/pppppppp/8/8/8/7P/PPPPPPP1/RNBQKBNR b KQkq - 0 1",
            ),
            // knights
            (
                "B1->A3",
                "rnbqkbnr/pppppppp/8/8/8/N7/PPPPPPPP/R1BQKBNR b KQkq - 1 1",
            ),
            (
                "B1->C3",
                "rnbqkbnr/pppppppp/8/8/8/2N5/PPPPPPPP/R1BQKBNR b KQkq - 1 1",
            ),
            (
                "G1->F3",
                "rnbqkbnr/pppppppp/8/8/8/5N2/PPPPPPPP/RNBQKB1R b KQkq - 1 1",
            ),
            (
                "G1->H3",
                "rnbqkbnr/pppppppp/8/8/8/7N/PPPPPPPP/RNBQKB1R b KQkq - 1 1",
            ),
        ];

        for (mv, fen_after) in inputs.iter() {
            let mut pos = Position::from_fen(fen);
            let mv = intmove_from_ascii(mv);
            pos.make_move(mv).unwrap();
            assert_eq!(fen_after.to_string(), pos.to_fen());
        }
    }

    #[test]
    fn make_move_change_colors() {
        use Player::*;

        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let mut pos = Position::from_fen(fen);
        assert_eq!(White, pos.to_move);

        let mv = intmove_from_ascii("E2->E4");
        pos.make_move(mv).unwrap();
        assert_eq!(Black, pos.to_move);

        let mv = intmove_from_ascii("E7->E5");
        pos.make_move(mv).unwrap();
        assert_eq!(White, pos.to_move);
    }

    #[test]
    fn make_move_change_full_moves() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let mut pos = Position::from_fen(fen);
        assert_eq!(1, pos.full_moves);

        // white moves
        let mv = intmove_from_ascii("E2->E4");
        pos.make_move(mv).unwrap();
        assert_eq!(1, pos.full_moves);

        // black moves
        let mv = intmove_from_ascii("E7->E5");
        pos.make_move(mv).unwrap();
        assert_eq!(2, pos.full_moves);

        // white moves
        let mv = intmove_from_ascii("D2->D4");
        pos.make_move(mv).unwrap();
        assert_eq!(2, pos.full_moves);

        // black moves
        let mv = intmove_from_ascii("D7->D5");
        pos.make_move(mv).unwrap();
        assert_eq!(3, pos.full_moves);
    }

    #[test]
    fn make_move_change_half_moves() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let mut pos = Position::from_fen(fen);
        assert_eq!(0, pos.half_moves);

        // white moves a pawn
        let mv = intmove_from_ascii("E2->E4");
        pos.make_move(mv).unwrap();
        assert_eq!(0, pos.half_moves);

        // black moves a pawn too
        let mv = intmove_from_ascii("E7->E5");
        pos.make_move(mv).unwrap();
        assert_eq!(0, pos.half_moves);

        // white moves a knight
        let mv = intmove_from_ascii("G1->F3");
        pos.make_move(mv).unwrap();
        assert_eq!(1, pos.half_moves); // FIXME:

        // black moves a knight too
        let mv = intmove_from_ascii("B8->C6");
        pos.make_move(mv).unwrap();
        assert_eq!(2, pos.half_moves);

        // white captures a pawn by the knight
        let mv = intmove_from_ascii("F3->E5");
        pos.make_move(mv).unwrap();
        assert_eq!(0, pos.half_moves);
    }

    #[test]
    fn make_move_starting_ep1() {
        let fen = "k7/8/8/4pP2/8/8/8/K7 w KQkq e6 0 1";
        let mut pos = Position::from_fen(fen);

        let mv = "F5->E6"; // ep.
        pos.make_move(intmove_from_ascii(mv)).unwrap();

        let result_fen = "k7/8/4P3/8/8/8/8/K7 b KQkq - 0 1";
        assert_eq!(pos.to_fen(), result_fen);
    }

    #[test]
    fn make_move_starting_ep() {
        let fen = "8/8/8/8/4Pp2/8/8/8 b KQkq e3 0 1";
        let mut pos = Position::from_fen(fen);

        let mv = "F4->E3"; // ep.
        pos.make_move(intmove_from_ascii(mv)).unwrap();

        let result_fen = "8/8/8/8/8/4p3/8/8 w KQkq - 0 2";
        assert_eq!(pos.to_fen(), result_fen);
    }

    #[test]
    fn make_move_lose_castling_rights_left_rook() {
        let fen = "6k1/8/8/8/8/8/8/R3K2R w KQ - 0 1";
        let mut pos = Position::from_fen(fen);

        assert!(pos.castle_rights.contains("K"));
        assert!(pos.castle_rights.contains("Q"));

        let mv = "A1->A2";
        pos.make_move(intmove_from_ascii(mv)).unwrap();

        let result_fen = "6k1/8/8/8/8/8/R7/4K2R b K - 1 1";

        assert_eq!(pos.to_fen(), result_fen);
        assert!(pos.castle_rights.contains("K"));
        assert!(!pos.castle_rights.contains("Q"));
    }

    #[test]
    fn make_move_lose_castling_rights_right_rook() {
        let fen = "6k1/8/8/8/8/8/8/R3K2R w KQ - 0 1";
        let mut pos = Position::from_fen(fen);

        assert!(pos.castle_rights.contains("K"));
        assert!(pos.castle_rights.contains("Q"));

        let mv = "H1->H2";
        pos.make_move(intmove_from_ascii(mv)).unwrap();

        let result_fen = "6k1/8/8/8/8/8/7R/R3K3 b Q - 1 1";

        assert_eq!(pos.to_fen(), result_fen);
        assert!(!pos.castle_rights.contains("K"));
        assert!(pos.castle_rights.contains("Q"));
    }

    #[test]
    fn make_move_lose_castling_rights_king() {
        let fen = "6k1/8/8/8/8/8/8/R3K2R w KQ - 0 1";
        let mut pos = Position::from_fen(fen);

        assert!(pos.castle_rights.contains("K"));
        assert!(pos.castle_rights.contains("Q"));

        let mv = "E1->E2";
        pos.make_move(intmove_from_ascii(mv)).unwrap();

        let result_fen = "6k1/8/8/8/8/8/4K3/R6R b - - 1 1";

        assert_eq!(pos.to_fen(), result_fen);
        assert!(!pos.castle_rights.contains("K"));
        assert!(!pos.castle_rights.contains("Q"));
    }

    #[test]
    fn make_move_kingside_castling() {
        let fen = "6k1/8/8/8/8/8/8/R3K2R w KQ - 0 1";
        let mut pos = Position::from_fen(fen);

        assert!(pos.castle_rights.contains("K"));
        assert!(pos.castle_rights.contains("Q"));

        let mv = "E1->G1";
        pos.make_move(intmove_from_ascii(mv)).unwrap();

        let result_fen = "6k1/8/8/8/8/8/8/R4RK1 b - - 1 1";
        assert_eq!(pos.to_fen(), result_fen);
        assert!(!pos.castle_rights.contains("K"));
        assert!(!pos.castle_rights.contains("Q"));
    }

    #[test]
    fn make_move_queenside_castling() {
        let fen = "6k1/8/8/8/8/8/8/R3K2R w KQ - 0 1";
        let mut pos = Position::from_fen(fen);

        assert!(pos.castle_rights.contains("K"));
        assert!(pos.castle_rights.contains("Q"));

        let mv = "E1->C1";
        pos.make_move(intmove_from_ascii(mv)).unwrap();

        let result_fen = "6k1/8/8/8/8/8/8/2KR3R b - - 1 1";
        assert_eq!(pos.to_fen(), result_fen);
        assert!(!pos.castle_rights.contains("K"));
        assert!(!pos.castle_rights.contains("Q"));
    }

    // TODO: ROOK CAPTURED = lose rights
    #[test]
    fn make_move_losing_the_rook_loses_castling() {
        let fen = "rnbqk1nr/pppppp1p/6pb/8/8/1P6/PBPPPPPP/RN1QKBNR w KQkq - 2 3";
        let mut pos = Position::from_fen(fen);

        assert!(pos.castle_rights.contains("k"));
        assert!(pos.castle_rights.contains("q"));

        let mv = "B2->H8";
        pos.make_move(intmove_from_ascii(mv)).unwrap();

        let result_fen = "rnbqk1nB/pppppp1p/6pb/8/8/1P6/P1PPPPPP/RN1QKBNR b KQq - 0 3";
        assert_eq!(pos.to_fen(), result_fen);
        assert!(!pos.castle_rights.contains("k"));
        assert!(pos.castle_rights.contains("q"));
    }

    #[test]
    fn make_move_losing_the_rook_loses_castling2() {
        let fen = "rn1qkbnr/p1pppppp/bp6/8/8/6P1/PPPPPPBP/RNBQK1NR w KQkq - 2 3";
        let mut pos = Position::from_fen(fen);

        assert!(pos.castle_rights.contains("k"));
        assert!(pos.castle_rights.contains("q"));

        let mv = "G2->A8";
        pos.make_move(intmove_from_ascii(mv)).unwrap();

        let result_fen = "Bn1qkbnr/p1pppppp/bp6/8/8/6P1/PPPPPP1P/RNBQK1NR b KQk - 0 3";
        assert_eq!(pos.to_fen(), result_fen);
        assert!(pos.castle_rights.contains("k"));
        assert!(!pos.castle_rights.contains("q"));
    }

    #[test]
    fn regression() {
        let fen = "rnbqkbnr/ppppppp1/8/7p/8/3P4/PPP1PPPP/RNBQKBNR w KQkq h6 0 2";
        let mut pos = Position::from_fen(fen);
        let mv = "C1->H6";

        // let result_fen = "rnbqkbnr/ppppppp1/8/7p/8/3P4/PPP1PPPP/RNBQKBNp w KQkq h6 0 2";
        pos.make_move(intmove_from_ascii(mv)).unwrap();
        pos.unmake_move(intmove_from_ascii(mv)).unwrap();
        assert_eq!(pos.to_fen(), fen);
    }
}

#[cfg(test)]
mod test_unmoves {

    use super::*;

    #[test]
    fn unmake_move_starting() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

        let inputs = [
            // pawns (2 squares)
            (
                "A2->A4",
                "rnbqkbnr/pppppppp/8/8/P7/8/1PPPPPPP/RNBQKBNR b KQkq a3 0 1",
            ),
            (
                "B2->B4",
                "rnbqkbnr/pppppppp/8/8/1P6/8/P1PPPPPP/RNBQKBNR b KQkq b3 0 1",
            ),
            (
                "C2->C4",
                "rnbqkbnr/pppppppp/8/8/2P5/8/PP1PPPPP/RNBQKBNR b KQkq c3 0 1",
            ),
            (
                "D2->D4",
                "rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq d3 0 1",
            ),
            (
                "E2->E4",
                "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
            ),
            (
                "F2->F4",
                "rnbqkbnr/pppppppp/8/8/5P2/8/PPPPP1PP/RNBQKBNR b KQkq f3 0 1",
            ),
            (
                "G2->G4",
                "rnbqkbnr/pppppppp/8/8/6P1/8/PPPPPP1P/RNBQKBNR b KQkq g3 0 1",
            ),
            (
                "H2->H4",
                "rnbqkbnr/pppppppp/8/8/7P/8/PPPPPPP1/RNBQKBNR b KQkq h3 0 1",
            ),
            // pawns (1 square)
            (
                "A2->A3",
                "rnbqkbnr/pppppppp/8/8/8/P7/1PPPPPPP/RNBQKBNR b KQkq - 0 1",
            ),
            (
                "B2->B3",
                "rnbqkbnr/pppppppp/8/8/8/1P6/P1PPPPPP/RNBQKBNR b KQkq - 0 1",
            ),
            (
                "C2->C3",
                "rnbqkbnr/pppppppp/8/8/8/2P5/PP1PPPPP/RNBQKBNR b KQkq - 0 1",
            ),
            (
                "D2->D3",
                "rnbqkbnr/pppppppp/8/8/8/3P4/PPP1PPPP/RNBQKBNR b KQkq - 0 1",
            ),
            (
                "E2->E3",
                "rnbqkbnr/pppppppp/8/8/8/4P3/PPPP1PPP/RNBQKBNR b KQkq - 0 1",
            ),
            (
                "F2->F3",
                "rnbqkbnr/pppppppp/8/8/8/5P2/PPPPP1PP/RNBQKBNR b KQkq - 0 1",
            ),
            (
                "G2->G3",
                "rnbqkbnr/pppppppp/8/8/8/6P1/PPPPPP1P/RNBQKBNR b KQkq - 0 1",
            ),
            (
                "H2->H3",
                "rnbqkbnr/pppppppp/8/8/8/7P/PPPPPPP1/RNBQKBNR b KQkq - 0 1",
            ),
            // knights
            (
                "B1->A3",
                "rnbqkbnr/pppppppp/8/8/8/N7/PPPPPPPP/R1BQKBNR b KQkq - 1 1",
            ),
            (
                "B1->C3",
                "rnbqkbnr/pppppppp/8/8/8/2N5/PPPPPPPP/R1BQKBNR b KQkq - 1 1",
            ),
            (
                "G1->F3",
                "rnbqkbnr/pppppppp/8/8/8/5N2/PPPPPPPP/RNBQKB1R b KQkq - 1 1",
            ),
            (
                "G1->H3",
                "rnbqkbnr/pppppppp/8/8/8/7N/PPPPPPPP/RNBQKB1R b KQkq - 1 1",
            ),
        ];

        for (mv, fen_after) in inputs.iter() {
            let mut pos = Position::from_fen(fen);
            let mv = intmove_from_ascii(mv);
            pos.make_move(mv).unwrap();
            assert_eq!(fen_after.to_string(), pos.to_fen());

            pos.unmake_move(mv).unwrap();
            assert_eq!(fen, pos.to_fen());
        }
    }


    #[test]
    fn unmake_move_starting2() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

        let mut pos = Position::from_fen(fen);
        let moves = pos.legal_moves();

        assert_eq!(fen, pos.to_fen());
        for mv in moves.iter() {
            pos.make_move(*mv).unwrap();
            pos.unmake_move(*mv).unwrap();
            assert_eq!(fen, pos.to_fen());
        }
    }

    #[test]
    fn unmake_move_starting_depth2() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

        let mut pos = Position::from_fen(fen);
        let moves = pos.legal_moves();

        assert_eq!(fen, pos.to_fen());
        for mv in moves.iter() {
            pos.make_move(*mv).unwrap();
            let moves2 = pos.legal_moves();
            for mv2 in moves2.iter() {
                pos.make_move(*mv2).unwrap();
                pos.unmake_move(*mv2).unwrap();
            }
            pos.unmake_move(*mv).unwrap();
            assert_eq!(fen, pos.to_fen());
        }
    }

    #[test]
    fn unmake_move_starting_depth3() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

        let mut pos = Position::from_fen(fen);
        let moves = pos.legal_moves();

        assert_eq!(fen, pos.to_fen());
        for mv in moves.iter() {
            pos.make_move(*mv).unwrap();

            let before = pos.to_fen();

            let moves2 = pos.legal_moves();
            for mv2 in moves2.iter() {
                let before2 = pos.to_fen();

                pos.make_move(*mv2).unwrap();
                let moves3 = pos.legal_moves();
                for mv3 in moves3.iter() {
                    let before3 = pos.to_fen();

                    pos.make_move(*mv3).unwrap();
                    pos.unmake_move(*mv3).unwrap();

                    let after3 = pos.to_fen();
                    assert_eq!(before3, after3);
                }
                pos.unmake_move(*mv2).unwrap();
                let after2 = pos.to_fen();
                assert_eq!(before2, after2);
            }
            let after = pos.to_fen();
            assert_eq!(before, after);

            pos.unmake_move(*mv).unwrap();
            assert_eq!(fen, pos.to_fen());
        }
    }


    #[test]
    fn unmake_move_change_colors() {
        use Player::*;

        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let mut pos = Position::from_fen(fen);
        assert_eq!(White, pos.to_move);

        let mv = intmove_from_ascii("E2->E4");
        pos.make_move(mv).unwrap();
        assert_eq!(Black, pos.to_move);

        let mv2 = intmove_from_ascii("E7->E5");
        pos.make_move(mv2).unwrap();
        assert_eq!(White, pos.to_move);

        pos.unmake_move(mv2).unwrap();
        assert_eq!(Black, pos.to_move);

        pos.unmake_move(mv).unwrap();
        assert_eq!(White, pos.to_move);
    }

    #[test]
    fn unmake_move_change_full_moves() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let mut pos = Position::from_fen(fen);
        assert_eq!(1, pos.full_moves);

        // white moves
        let mv = intmove_from_ascii("E2->E4");
        pos.make_move(mv).unwrap();
        assert_eq!(1, pos.full_moves);

        // black moves
        let mv2 = intmove_from_ascii("E7->E5");
        pos.make_move(mv2).unwrap();
        assert_eq!(2, pos.full_moves);

        // white moves
        let mv3 = intmove_from_ascii("D2->D4");
        pos.make_move(mv3).unwrap();
        assert_eq!(2, pos.full_moves);

        // black moves
        let mv4 = intmove_from_ascii("D7->D5");
        pos.make_move(mv4).unwrap();
        assert_eq!(3, pos.full_moves);

        // black move unmake
        pos.unmake_move(mv4).unwrap();
        assert_eq!(2, pos.full_moves);

        // white move unmake
        pos.unmake_move(mv3).unwrap();
        assert_eq!(2, pos.full_moves);

        // black move unmake
        pos.unmake_move(mv2).unwrap();
        assert_eq!(1, pos.full_moves);

        // white move unmake
        pos.unmake_move(mv).unwrap();
        assert_eq!(1, pos.full_moves);
    }

    #[test]
    fn unmake_move_change_half_moves() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let mut pos = Position::from_fen(fen);
        assert_eq!(0, pos.half_moves);

        // white moves a pawn
        let mv = intmove_from_ascii("E2->E4");
        pos.make_move(mv).unwrap();
        assert_eq!(0, pos.half_moves);

        // black moves a pawn too
        let mv2 = intmove_from_ascii("E7->E5");
        pos.make_move(mv2).unwrap();
        assert_eq!(0, pos.half_moves);

        // white moves a knight
        let mv3 = intmove_from_ascii("G1->F3");
        pos.make_move(mv3).unwrap();
        assert_eq!(1, pos.half_moves);

        // black moves a knight too
        let mv4 = intmove_from_ascii("B8->C6");
        pos.make_move(mv4).unwrap();
        assert_eq!(2, pos.half_moves);

        // white captures a pawn by the knight
        let mv5 = intmove_from_ascii("F3->E5");
        pos.make_move(mv5).unwrap();
        assert_eq!(0, pos.half_moves);

        // white undoes the knight capturing a pawn
        pos.unmake_move(mv5).unwrap();
        assert_eq!(2, pos.half_moves);

        // black unmoves a knight too
        pos.unmake_move(mv4).unwrap();
        assert_eq!(1, pos.half_moves);

        // white unmoves a knight
        pos.unmake_move(mv3).unwrap();
        assert_eq!(0, pos.half_moves);

        pos.unmake_move(mv2).unwrap();
        assert_eq!(0, pos.half_moves);

        pos.unmake_move(mv).unwrap();
        assert_eq!(0, pos.half_moves);
    }

    #[test]
    fn unmake_move_starting_knight() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let mut pos = Position::from_fen(fen);

        let mv = "B1->C3"; // knight
        pos.make_move(intmove_from_ascii(mv)).unwrap();

        let result_fen = "rnbqkbnr/pppppppp/8/8/8/2N5/PPPPPPPP/R1BQKBNR b KQkq - 1 1";
        assert_eq!(pos.to_fen(), result_fen);

        pos.unmake_move(intmove_from_ascii(mv)).unwrap();
        assert_eq!(pos.to_fen(), fen);
    }


    #[test]
    fn unmake_move_starting_ep1() {
        let fen = "k7/8/8/4pP2/8/8/8/K7 w KQkq e6 0 1";
        let mut pos = Position::from_fen(fen);

        let mv = "F5->E6"; // ep.
        pos.make_move(intmove_from_ascii(mv)).unwrap();

        let result_fen = "k7/8/4P3/8/8/8/8/K7 b KQkq - 0 1";
        assert_eq!(pos.to_fen(), result_fen);

        pos.unmake_move(intmove_from_ascii(mv)).unwrap();
        assert_eq!(pos.to_fen(), fen);
    }

    #[test]
    fn unmake_move_starting_ep() {
        let fen = "8/8/8/8/4Pp2/8/8/8 b KQkq e3 0 1";
        let mut pos = Position::from_fen(fen);

        let mv = "F4->E3"; // ep.
        pos.make_move(intmove_from_ascii(mv)).unwrap();

        let result_fen = "8/8/8/8/8/4p3/8/8 w KQkq - 0 2";
        assert_eq!(pos.to_fen(), result_fen);

        pos.unmake_move(intmove_from_ascii(mv)).unwrap();
        assert_eq!(pos.to_fen(), fen);
    }

    #[test]
    fn unmake_move_lose_castling_rights_left_rook() {
        let fen = "6k1/8/8/8/8/8/8/R3K2R w KQ - 0 1";
        let mut pos = Position::from_fen(fen);

        assert!(pos.castle_rights.contains("K"));
        assert!(pos.castle_rights.contains("Q"));

        let mv = "A1->A2";
        pos.make_move(intmove_from_ascii(mv)).unwrap();

        let result_fen = "6k1/8/8/8/8/8/R7/4K2R b K - 1 1";

        assert_eq!(pos.to_fen(), result_fen);
        assert!(pos.castle_rights.contains("K"));
        assert!(!pos.castle_rights.contains("Q"));

        pos.unmake_move(intmove_from_ascii(mv)).unwrap();
        assert_eq!(pos.to_fen(), fen);
    }

    #[test]
    fn unmake_move_lose_castling_rights_right_rook() {
        let fen = "6k1/8/8/8/8/8/8/R3K2R w KQ - 0 1";
        let mut pos = Position::from_fen(fen);

        assert!(pos.castle_rights.contains("K"));
        assert!(pos.castle_rights.contains("Q"));

        let mv = "H1->H2";
        pos.make_move(intmove_from_ascii(mv)).unwrap();

        let result_fen = "6k1/8/8/8/8/8/7R/R3K3 b Q - 1 1";

        assert_eq!(pos.to_fen(), result_fen);
        assert!(!pos.castle_rights.contains("K"));
        assert!(pos.castle_rights.contains("Q"));

        pos.unmake_move(intmove_from_ascii(mv)).unwrap();
        assert_eq!(pos.to_fen(), fen);
    }

    #[test]
    fn unmake_move_lose_castling_rights_king() {
        let fen = "6k1/8/8/8/8/8/8/R3K2R w KQ - 0 1";
        let mut pos = Position::from_fen(fen);

        assert!(pos.castle_rights.contains("K"));
        assert!(pos.castle_rights.contains("Q"));

        let mv = "E1->E2";
        pos.make_move(intmove_from_ascii(mv)).unwrap();

        let result_fen = "6k1/8/8/8/8/8/4K3/R6R b - - 1 1";

        assert_eq!(pos.to_fen(), result_fen);
        assert!(!pos.castle_rights.contains("K"));
        assert!(!pos.castle_rights.contains("Q"));

        pos.unmake_move(intmove_from_ascii(mv)).unwrap();
        assert_eq!(pos.to_fen(), fen);
    }

    #[test]
    fn unmake_move_kingside_castling() {
        let fen = "6k1/8/8/8/8/8/8/R3K2R w KQ - 0 1";
        let mut pos = Position::from_fen(fen);

        assert!(pos.castle_rights.contains("K"));
        assert!(pos.castle_rights.contains("Q"));

        let mv = "E1->G1";
        pos.make_move(intmove_from_ascii(mv)).unwrap();

        let result_fen = "6k1/8/8/8/8/8/8/R4RK1 b - - 1 1";
        assert_eq!(pos.to_fen(), result_fen);
        assert!(!pos.castle_rights.contains("K"));
        assert!(!pos.castle_rights.contains("Q"));

        pos.unmake_move(intmove_from_ascii(mv)).unwrap();
        assert_eq!(pos.to_fen(), fen);
    }

    #[test]
    fn unmake_move_queenside_castling() {
        let fen = "6k1/8/8/8/8/8/8/R3K2R w KQ - 0 1";
        let mut pos = Position::from_fen(fen);

        assert!(pos.castle_rights.contains("K"));
        assert!(pos.castle_rights.contains("Q"));

        let mv = "E1->C1";
        pos.make_move(intmove_from_ascii(mv)).unwrap();

        let result_fen = "6k1/8/8/8/8/8/8/2KR3R b - - 1 1";
        assert_eq!(pos.to_fen(), result_fen);
        assert!(!pos.castle_rights.contains("K"));
        assert!(!pos.castle_rights.contains("Q"));

        pos.unmake_move(intmove_from_ascii(mv)).unwrap();
        assert_eq!(pos.to_fen(), fen);
    }

    #[test]
    fn unmake_move_losing_the_rook_loses_castling() {
        let fen = "rnbqk1nr/pppppp1p/6pb/8/8/1P6/PBPPPPPP/RN1QKBNR w KQkq - 2 3";
        let mut pos = Position::from_fen(fen);

        assert!(pos.castle_rights.contains("k"));
        assert!(pos.castle_rights.contains("q"));

        let mv = "B2->H8";
        pos.make_move(intmove_from_ascii(mv)).unwrap();

        let result_fen = "rnbqk1nB/pppppp1p/6pb/8/8/1P6/P1PPPPPP/RN1QKBNR b KQq - 0 3";
        assert_eq!(pos.to_fen(), result_fen);
        assert!(!pos.castle_rights.contains("k"));
        assert!(pos.castle_rights.contains("q"));

        pos.unmake_move(intmove_from_ascii(mv)).unwrap();
        assert_eq!(pos.to_fen(), fen);
    }

    #[test]
    fn unmake_move_losing_the_rook_loses_castling2() {
        let fen = "rn1qkbnr/p1pppppp/bp6/8/8/6P1/PPPPPPBP/RNBQK1NR w KQkq - 2 3";
        let mut pos = Position::from_fen(fen);

        assert!(pos.castle_rights.contains("k"));
        assert!(pos.castle_rights.contains("q"));

        let mv = "G2->A8";
        pos.make_move(intmove_from_ascii(mv)).unwrap();

        let result_fen = "Bn1qkbnr/p1pppppp/bp6/8/8/6P1/PPPPPP1P/RNBQK1NR b KQk - 0 3";
        assert_eq!(pos.to_fen(), result_fen);
        assert!(pos.castle_rights.contains("k"));
        assert!(!pos.castle_rights.contains("q"));

        pos.unmake_move(intmove_from_ascii(mv)).unwrap();
        assert_eq!(pos.to_fen(), fen);
    }
}
