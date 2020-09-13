#[path = "parser.rs"]
pub mod parser;

pub use parser::*;

impl Position {
    fn line(&self, src: Coord, dx: i32, dy: i32, all_moves: &mut Vec<Move>) {
        let RowCol {
            row: src_row,
            col: src_col,
        } = coord2rowcol(src);

        let mut dest_row = src_row + dx;
        let mut dest_col = src_col + dy;

        let color = self[src].unwrap().player;
        let opp_color = color.opposite();

        while let Some(dest) = rowcol2coord_safe(dest_row, dest_col) {
            let dest_field = self[dest];

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

    fn try_add(&self, src: Coord, dest_row: i32, dest_col: i32, all_moves: &mut Vec<Move>) {
        self.try_add_pawn(src, dest_row, dest_col, all_moves, true);
    }

    fn try_add_pawn(
        &self,
        src: Coord,
        dest_row: i32,
        dest_col: i32,
        all_moves: &mut Vec<Move>,
        capture_ok: bool,
    ) {
        if let Some(dest) = rowcol2coord_safe(dest_row, dest_col) {
            let PlayerPiece {
                player: color,
                piece,
            } = self[src].unwrap();
            let opp_color = color.opposite();
            let dest_field = self[dest];

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

    fn generate_moves_from(&self, src: Coord, piece: Piece, color: Player) -> Vec<Move> {
        assert_eq!(PlayerPiece::new(color, piece), self[src].unwrap());

        let RowCol {
            row: src_row,
            col: src_col,
        } = coord2rowcol(src);

        let row_delta: i32 = if color == Player::White { 1 } else { -1 };

        let mut all_moves = vec![];

        // TODO: implement castling

        match piece {
            Piece::Pawn => {
                // TODO: implement en passant

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
                    if let Some(dest) = rowcol2coord_safe(dest_row, dest_col) {
                        let dest_piece = self[dest];

                        let en_passant_ok =
                            self.en_passant.is_some() && self.en_passant.unwrap() == dest;

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

        for coord in COORDS.iter() {
            if let Some(player_piece) = self[coord] {
                if player_piece.player == color {
                    all_moves.append(&mut self.generate_moves_from(
                        coord,
                        player_piece.piece,
                        color,
                    ));
                }
            }
        }

        all_moves
    }

    fn king_location(&self, player: Player) -> Option<Coord> {
        let king = PlayerPiece {
            player,
            piece: Piece::King,
        };
        for coord in COORDS.iter() {
            if self[coord] == Some(king) {
                return Some(coord);
            }
        }
        return None;
    }

    pub fn fields_attacked_by(&self, player: Player) -> Vec<Coord> {
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

    pub fn legal_moves(&self) -> Vec<Move> {
        let moves = self.moves();
        let mut result = vec![];
        for mv in moves.iter() {
            let mut pos = self.clone();
            pos.make_move(*mv).unwrap();
            if !pos.is_king_in_check(self.to_move) {
                result.push(*mv);
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
