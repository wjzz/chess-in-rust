#[path = "make_move.rs"]
pub mod make_move;

pub use make_move::*;

impl Position {
    fn line(&self, color: Player, src: usize, deltas: &[i32], all_moves: &mut Vec<IntMove>) {
        assert!(src < MAX_INDEX88);

        for &delta in deltas {
            let mut dest = src as i32 + delta;
            while dest & 0x88 == 0 {
                assert!(0 <= dest && dest < MAX_INDEX88 as i32);

                let dest_field = self.board[dest as usize];

                // println!("Trying to go from {} to {}", index2coord(src), index2coord(dest as usize));

                if dest_field == EMPTY || boardcell_player(dest_field) != color {
                    all_moves.push(intmove_encode(src, dest as usize, None));

                    if dest_field != EMPTY {
                        break;
                    }
                } else {
                    break;
                }

                dest += delta;
            }
        }
    }

    fn try_add_non_pawn(
        &self,
        color: Player,
        src: usize,
        dest: i32,
        all_moves: &mut Vec<IntMove>,
    ) {
        if dest & 0x88 == 0 {
            assert!(dest >= 0);
            let dest_field = self.board[dest as usize];

            if dest_field == EMPTY || boardcell_player(dest_field) != color {
                all_moves.push(intmove_encode(src, dest as usize, None));
            }
        }
    }

    fn try_add_pawn(
        &self,
        src: usize,
        dest: i32,
        all_moves: &mut Vec<IntMove>,
        capture_ok: bool
    ) {
        if dest & 0x88 == 0 {
            assert!(dest >= 0);
            let color = boardcell_player(self.board[src]);
            let dest_field = self.board[dest as usize];

            if dest_field == EMPTY || (capture_ok && boardcell_player(dest_field) != color) {
                let reaches_last_row = match color {
                    Player::White => dest >= 112, //dest_row == 7,
                    Player::Black => dest <= 7, //dest_row == 0,
                };

                if reaches_last_row {
                    for &promo_piece in PROMOTABLE_PIECES.iter() {
                        all_moves.push(intmove_encode(src, dest as usize, Some(promo_piece)));
                    }
                } else {
                    all_moves.push(intmove_encode(src, dest as usize, None));
                }
            }
        }
    }

    fn generate_moves_from(&self, src: usize, piece: Piece, color: Player, all_moves: &mut Vec<IntMove>) {
        assert_eq!(boardcell_encode(color, piece), self.board[src]);

        match piece {
            Piece::Pawn => {
                let is_first_move = match color {
                    Player::White => 16 <= src && src <= 23,
                    Player::Black => 96 <= src && src <= 103,
                };

                let delta: i32 = if color == Player::White { 16 } else { -16 };
                let square_in_front = src as i32 + delta;

                self.try_add_pawn(src, square_in_front, all_moves, false);

                // first move by two squares
                if is_first_move {
                    // make sure the square before the pawn is empty!
                    if self.board[square_in_front as usize] == EMPTY {
                        self.try_add_pawn(
                            src,
                            src as i32 + 2 * delta,
                            all_moves,
                            false,
                        );
                    }
                }

                // captures
                for col_delta in [-1, 1].iter() {
                    let dest = src as i32 + delta + col_delta;
                    if dest & 0x88 == 0 {
                        let dest_piece = self.board[dest as usize];

                        let en_passant_ok = self.en_passant.is_some()
                            && self.en_passant.unwrap() == dest as usize;

                        if en_passant_ok
                            || (!en_passant_ok
                                && dest_piece != EMPTY
                                && boardcell_player(dest_piece) != color)
                        {
                            self.try_add_pawn(
                                src,
                                dest,
                                all_moves,
                                true,
                            );
                        }
                    }
                }
            }
            Piece::King => {
                for delta in &[ 16, 17, 1, -15, -16, -17, -1, 15] {
                    self.try_add_non_pawn(color, src, src as i32 + delta, all_moves);
                }

                // castling
                let king_initial_coord = if color == Player::White { 4 } else { 116 };

                if src == king_initial_coord {
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

                    // castling kingside
                    if self.castle_rights.contains(&ascii_k) {
                        let (rook_dest, f1, f2) = match color {
                            Player::White => (7, 5, 6),
                            Player::Black => (119, 117, 118),
                        };

                        assert_eq!(self.board[rook_dest], boardcell_encode(color, Piece::Rook));

                        if self.board[f1] == EMPTY && self.board[f2] == EMPTY {
                            self.try_add_non_pawn(color, src, f2 as i32, all_moves);
                        }
                    }

                    // castling queenside
                    if self.castle_rights.contains(&ascii_q) {
                        let (rook_dest, f1, f2, f3) = match color {
                            Player::White => (0, 1, 2, 3),
                            Player::Black => (112, 113, 114, 115),
                        };

                        assert_eq!(self.board[rook_dest], boardcell_encode(color, Piece::Rook));

                        if self.board[f1] == EMPTY && self.board[f2] == EMPTY && self.board[f3] == EMPTY {
                            // self.try_add_flag(src, f2 as i32, all_moves, CASTLE_FLAG);
                            self.try_add_non_pawn(color, src, f2 as i32, all_moves);

                        }
                    }
                }
            }
            Piece::Knight => {
                for delta in &[ 31, 33, -31, -33, 18, -14, -18, 14] {
                    self.try_add_non_pawn(color, src, src as i32 + delta, all_moves);
                }
            }
            Piece::Queen => {
                let deltas = &[17, -15, -17, 15, 16, 1, -16, -1];
                self.line(color, src, deltas, all_moves);
            }
            Piece::Bishop => {
                let deltas = &[17, -15, -17, 15];
                self.line(color, src, deltas, all_moves);
            }
            Piece::Rook => {
                let deltas = &[16, 1, -16, -1];
                self.line(color, src, deltas, all_moves);
            }
        }
    }

    pub fn moves(&self) -> Vec<IntMove> {
        self.moves_by(self.to_move)
    }

    pub fn moves_by(&self, color: Player) -> Vec<IntMove> {
        let mut all_moves = vec![];

        for &index in INDEXES88.iter() {
            let player_piece = self.board[index];
            if player_piece != EMPTY {
                if boardcell_player(player_piece) == color {
                    self.generate_moves_from(
                        index,
                        boardcell_piece(player_piece),
                        color,
                        &mut all_moves);
                    }
                }
        }

        all_moves
    }

    pub fn fields_attacked_by(&self, player: Player) -> Vec<usize> {
        // TODO: use a set here
        self.moves_by(player)
            .iter()
            .map(|mv: &IntMove| intmove_dest(*mv))
            .collect()
    }

    pub fn is_king_in_check(&self, player: Player) -> bool {
        let king_coord = self.kings[player as usize];
        for mv in self.moves_by(player.opposite()).iter() {
            if intmove_dest(*mv) == king_coord {
                return true;
            }
        }
        return false;
    }

    pub fn is_castling_move_color(&self, mv: IntMove, color: Player) -> bool {
        let (src, dest, _promote_to) = intmove_destructure(mv);

        let src = index2coord(src);
        let dest = index2coord(dest);
        match color {
            Player::White => src == "E1" && (dest == "G1" || dest == "C1"),
            Player::Black => src == "E8" && (dest == "G8" || dest == "C8"),
        }
    }

    pub fn is_castling_move(&self, mv: IntMove) -> bool {
        return self.is_castling_move_color(mv, self.to_move);
    }

    fn fields_to_check_castling(&self, mv: IntMove) -> Vec<Coord> {
        let dest = index2coord(intmove_dest(mv));
        match (self.to_move, dest) {
            (Player::White, "G1") => vec!["F1", "G1"],
            (Player::White, "C1") => vec!["D1", "C1"],
            (Player::Black, "G8") => vec!["F8", "G8"],
            (Player::Black, "C8") => vec!["D8", "C8"],
            _ => panic!("Incorrect castling move {:?}", mv),
        }
    }

    pub fn rook_position_castling(&self, mv: IntMove) -> (Coord, Coord) {
        self.rook_position_castling_color(mv, self.to_move)
    }

    pub fn rook_position_castling_color(&self, mv: IntMove, color: Player) -> (Coord, Coord) {
        // TODO: remove index calculations
        let dest = index2coord(intmove_dest(mv));
        match (color, dest) {
            (Player::White, "G1") => ("H1", "F1"),
            (Player::White, "C1") => ("A1", "D1"),
            (Player::Black, "G8") => ("H8", "F8"),
            (Player::Black, "C8") => ("A8", "D8"),
            _ => panic!("Incorrect castling move {:?}", mv),
        }
    }

    fn can_safely_castle(&self, mv: IntMove) -> bool {
        let fields = self.fields_to_check_castling(mv);
        let attacked_fields = self.fields_attacked_by(self.to_move.opposite());
        for coord in fields.iter() {
            // make sure coord in not attacked
            let index = coord2index(coord);
            if attacked_fields.contains(&index) {
                return false;
            }
            // check opponent's pawns
            let deltas = [15, 17];
            let sign = if self.to_move == Player::White { 1 } else { -1 };
            let opp_pawn = B_PAWN * sign;
            for delta in deltas.iter() {
                let dest = index as i32 + delta * sign;
                if dest & 0x88 == 0 && self.board[dest as usize] == opp_pawn {
                    return false;
                }
            }
        }
        return true;
    }

    pub fn legal_moves(&mut self) -> Vec<IntMove> {
        let moves = self.moves();
        let mut result = vec![];
        for mv in moves.iter() {
            if self.is_castling_move(*mv) {
                if !self.is_king_in_check(self.to_move) && self.can_safely_castle(*mv) {
                    result.push(*mv);
                }
            } else {
                let to_move = self.to_move;
                self.make_move(*mv).unwrap();
                if !self.is_king_in_check(to_move) {
                    result.push(*mv);
                }
                self.unmake_move(*mv).unwrap();
            }
        }
        result
    }

    pub fn is_checkmate(&mut self) -> bool {
        self.legal_moves().len() == 0 && self.is_king_in_check(self.to_move)
    }

    pub fn is_stalemate(&mut self) -> bool {
        self.legal_moves().len() == 0 && !self.is_king_in_check(self.to_move)
    }
}
