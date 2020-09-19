#[path = "parser.rs"]
pub mod parser;

pub use parser::*;

impl Position {
    fn line(&self, src: usize, dx: i32, dy: i32, all_moves: &mut Vec<Move>) {
        let RowCol {
            row: src_row,
            col: src_col,
        } = index2rowcol(src);

        let mut dest_row = src_row + dx;
        let mut dest_col = src_col + dy;

        let color = self.board[src].unwrap().player;
        let opp_color = color.opposite();

        while let Some(dest) = rowcol2index_safe(dest_row, dest_col) {
            let dest_field = self.board[dest];

            if dest_field.is_none() || dest_field.unwrap().player == opp_color {
                all_moves.push(Move::new(src, dest, None));

                if dest_field.is_some() {
                    return;
                }
            } else {
                return;
            }

            dest_row += dx;
            dest_col += dy;
        }
    }

    fn try_add(&self, src: usize, dest_row: i32, dest_col: i32, all_moves: &mut Vec<Move>) {
        self.try_add_pawn(src, dest_row, dest_col, all_moves, true);
    }

    fn try_add_pawn(
        &self,
        src: usize,
        dest_row: i32,
        dest_col: i32,
        all_moves: &mut Vec<Move>,
        capture_ok: bool,
    ) {
        if let Some(dest) = rowcol2index_safe(dest_row, dest_col) {
            let PlayerPiece {
                player: color,
                piece,
            } = self.board[src].unwrap();
            let opp_color = color.opposite();
            let dest_field = self.board[dest];

            if dest_field.is_none() || (capture_ok && dest_field.unwrap().player == opp_color) {
                if piece != Piece::Pawn {
                    all_moves.push(Move::new(src, dest, None));
                } else {
                    let reaches_last_row = match color {
                        Player::White => dest_row == 7,
                        Player::Black => dest_row == 0,
                    };

                    if reaches_last_row {
                        for &promo_piece in PROMOTABLE_PIECES.iter() {
                            all_moves.push(Move::new(src, dest, Some(promo_piece)));
                        }
                    } else {
                        all_moves.push(Move::new(src, dest, None));
                    }
                }
            }
        }
    }

    fn generate_moves_from(&self, src: usize, piece: Piece, color: Player) -> Vec<Move> {
        assert_eq!(PlayerPiece::new(color, piece), self.board[src].unwrap());

        let RowCol {
            row: src_row,
            col: src_col,
        } = index2rowcol(src);

        let row_delta: i32 = if color == Player::White { 1 } else { -1 };

        let mut all_moves = vec![];

        match piece {
            Piece::Pawn => {
                let is_first_move = match color {
                    Player::White => src_row == 1,
                    Player::Black => src_row == 6,
                };

                self.try_add_pawn(src, src_row + row_delta, src_col, &mut all_moves, false);

                // first move by two squares
                if is_first_move {
                    // make sure the square before the pawn is empty!
                    let passing_square = rowcol2coord_safe(src_row + row_delta, src_col);
                    if let Some(passing) = passing_square {
                        if self[passing] == None {
                            self.try_add_pawn(
                                src,
                                src_row + row_delta * 2,
                                src_col,
                                &mut all_moves,
                                false,
                            );
                        }
                    }
                }

                // captures
                for col_delta in [-1, 1].iter() {
                    let dest_row = src_row + row_delta;
                    let dest_col = src_col + col_delta;
                    if let Some(dest) = rowcol2index_safe(dest_row, dest_col) {
                        let dest_piece = self.board[dest];

                        let en_passant_ok = self.en_passant.is_some()
                            && coord2index(self.en_passant.unwrap()) == dest;

                        if en_passant_ok
                            || (!en_passant_ok
                                && dest_piece.is_some()
                                && dest_piece.unwrap().player != color)
                        {
                            self.try_add_pawn(
                                src,
                                src_row + row_delta,
                                src_col + col_delta,
                                &mut all_moves,
                                true,
                            );
                        }
                    }
                }
            }
            Piece::King => {
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if dx != 0 || dy != 0 {
                            self.try_add(src, src_row + dy, src_col + dx, &mut all_moves);
                        }
                    }
                }

                // castling
                let king_initial_coord = if color == Player::White { "E1" } else { "E8" };

                if src == coord2index(king_initial_coord) {
                    let ascii_k = PlayerPiece {
                        piece: Piece::King,
                        player: color,
                    }
                    .to_ascii();
                    let ascii_q = PlayerPiece {
                        piece: Piece::Queen,
                        player: color,
                    }
                    .to_ascii();
                    let king_side_dx = 1;
                    let queen_side_dx = -1;

                    // castling kingside
                    if self.castle_rights.contains(&ascii_k) {
                        let rook_col = src_col + 3 * king_side_dx;
                        let rook_dest = rowcol2index(src_row, rook_col);

                        let test = self.board[rook_dest]
                            == Some(PlayerPiece {
                                player: color,
                                piece: Piece::Rook,
                            });

                        if !test {
                            println!("DEBUG: ");
                            println!("{}", self.to_ascii());
                            println!("FEN {}", self.to_fen());
                        }
                        assert!(
                            self.board[rook_dest]
                                == Some(PlayerPiece {
                                    player: color,
                                    piece: Piece::Rook
                                })
                        );

                        let free1 =
                            self.board[rowcol2index(src_row, src_col + king_side_dx)] == None;
                        let free2 =
                            self.board[rowcol2index(src_row, src_col + 2 * king_side_dx)] == None;

                        if free1 && free2 {
                            self.try_add(src, src_row, src_col + 2 * king_side_dx, &mut all_moves);
                        }
                    }

                    // castling queenside
                    if self.castle_rights.contains(&ascii_q) {
                        let rook_col = src_col + 4 * queen_side_dx;
                        let rook_dest = rowcol2index(src_row, rook_col);
                        assert!(
                            self.board[rook_dest]
                                == Some(PlayerPiece {
                                    player: color,
                                    piece: Piece::Rook
                                })
                        );

                        let free1 =
                            self.board[rowcol2index(src_row, src_col + queen_side_dx)] == None;
                        let free2 =
                            self.board[rowcol2index(src_row, src_col + 2 * queen_side_dx)] == None;
                        let free3 =
                            self.board[rowcol2index(src_row, src_col + 3 * queen_side_dx)] == None;

                        if free1 && free2 && free3 {
                            self.try_add(src, src_row, src_col + 2 * queen_side_dx, &mut all_moves);
                        }
                    }
                }
            }
            Piece::Knight => {
                for (w, d) in [(1, 2), (2, 1)].iter() {
                    for s1 in [-1, 1].iter() {
                        for s2 in [-1, 1].iter() {
                            let dx = w * s1;
                            let dy = d * s2;
                            self.try_add(src, src_row + dy, src_col + dx, &mut all_moves);
                        }
                    }
                }
            }
            Piece::Queen => {
                self.line(src, 0, -1, &mut all_moves);
                self.line(src, 0, 1, &mut all_moves);
                self.line(src, -1, 0, &mut all_moves);
                self.line(src, 1, 0, &mut all_moves);
                self.line(src, 1, -1, &mut all_moves);
                self.line(src, 1, 1, &mut all_moves);
                self.line(src, -1, -1, &mut all_moves);
                self.line(src, -1, 1, &mut all_moves);
            }
            Piece::Bishop => {
                self.line(src, 1, -1, &mut all_moves);
                self.line(src, 1, 1, &mut all_moves);
                self.line(src, -1, -1, &mut all_moves);
                self.line(src, -1, 1, &mut all_moves);
            }
            Piece::Rook => {
                self.line(src, 0, -1, &mut all_moves);
                self.line(src, 0, 1, &mut all_moves);
                self.line(src, -1, 0, &mut all_moves);
                self.line(src, 1, 0, &mut all_moves);
            }
        }

        all_moves
    }

    pub fn moves(&self) -> Vec<Move> {
        self.moves_by(self.to_move)
    }

    pub fn moves_by(&self, color: Player) -> Vec<Move> {
        let mut all_moves = vec![];

        for index in 0..64 {
            if let Some(player_piece) = self.board[index] {
                if player_piece.player == color {
                    all_moves.append(&mut self.generate_moves_from(
                        index,
                        player_piece.piece,
                        color,
                    ));
                }
            }
        }

        all_moves
    }

    fn king_location(&self, player: Player) -> Option<usize> {
        let king = PlayerPiece {
            player,
            piece: Piece::King,
        };
        for index in 0..64 {
            if self.board[index] == Some(king) {
                return Some(index);
            }
        }
        return None;
    }

    pub fn fields_attacked_by(&self, player: Player) -> Vec<usize> {
        // TODO: use a set here
        self.moves_by(player)
            .iter()
            .map(|mv: &Move| mv.dest)
            .collect()
    }

    pub fn is_king_in_check(&self, player: Player) -> bool {
        let king_coord = self.king_location(player);
        let fields_attacked_by_opp = self.fields_attacked_by(player.opposite());

        // TODO: we assume the king is on the board here
        fields_attacked_by_opp.contains(&king_coord.unwrap())
    }

    pub fn is_castling_move(&self, mv: Move) -> bool {
        let Move {
            src,
            dest,
            promote_to: _,
        } = mv;
        let src = index2coord(src);
        let dest = index2coord(dest);
        match self.to_move {
            Player::White => src == "E1" && (dest == "G1" || dest == "C1"),
            Player::Black => src == "E8" && (dest == "G8" || dest == "C8"),
        }
    }

    fn fields_to_check_castling(&self, mv: Move) -> Vec<Coord> {
        let dest = index2coord(mv.dest);
        match (self.to_move, dest) {
            (Player::White, "G1") => vec!["F1", "G1"],
            (Player::White, "C1") => vec!["D1", "C1"],
            (Player::Black, "G8") => vec!["F8", "G8"],
            (Player::Black, "C8") => vec!["D8", "C8"],
            _ => panic!("Incorrect castling move {:?}", mv),
        }
    }

    pub fn rook_position_castling(&self, mv: Move) -> (Coord, Coord) {
        let dest = index2coord(mv.dest);
        match (self.to_move, dest) {
            (Player::White, "G1") => ("H1", "F1"),
            (Player::White, "C1") => ("A1", "D1"),
            (Player::Black, "G8") => ("H8", "F8"),
            (Player::Black, "C8") => ("A8", "D8"),
            _ => panic!("Incorrect castling move {:?}", mv),
        }
    }

    fn can_safely_castle(&self, mv: Move) -> bool {
        let fields = self.fields_to_check_castling(mv);
        let attacked_fields = self.fields_attacked_by(self.to_move.opposite());
        for coord in fields.iter() {
            // make sure coord in not attacked
            let index = coord2index(coord);
            if attacked_fields.contains(&index) {
                return false;
            }
        }
        return true;
    }

    pub fn legal_moves(&self) -> Vec<Move> {
        let moves = self.moves();
        let mut result = vec![];
        for mv in moves.iter() {
            if self.is_castling_move(*mv) {
                if !self.is_king_in_check(self.to_move) && self.can_safely_castle(*mv) {
                    result.push(*mv);
                }
            } else {
                let mut pos = self.clone();
                pos.make_move(*mv).unwrap();
                if !pos.is_king_in_check(self.to_move) {
                    result.push(*mv);
                }
            }
        }
        result
    }

    pub fn is_checkmate(&self) -> bool {
        self.legal_moves().len() == 0 && self.is_king_in_check(self.to_move)
    }

    pub fn is_stalemate(&self) -> bool {
        self.legal_moves().len() == 0 && !self.is_king_in_check(self.to_move)
    }
}
