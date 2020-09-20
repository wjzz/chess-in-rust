#[path = "make_move.rs"]
pub mod make_move;

pub use make_move::*;

impl Position {
    fn line(&self, src: usize, deltas: &[i32], all_moves: &mut Vec<IntMove>) {
        assert!(src < MAX_INDEX88);

        // TODO: this should be given as param
        let color = boardcell_player(self.board[src]);
        let opp_color = color.opposite();

        for &delta in deltas {
            let mut dest = src as i32 + delta;
            while dest & 0x88 == 0 {
                assert!(0 <= dest && dest < MAX_INDEX88 as i32);

                let dest_field = self.board[dest as usize];

                // println!("Trying to go from {} to {}", index2coord(src), index2coord(dest as usize));

                if dest_field == EMPTY || boardcell_player(dest_field) == opp_color {
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

    fn try_add(&self, src: usize, dest: i32, all_moves: &mut Vec<IntMove>) {
        self.try_add_pawn(src, dest, all_moves, 0, true);
    }

    fn try_add_flag(&self, src: usize, dest: i32, all_moves: &mut Vec<IntMove>, flag: usize) {
        self.try_add_pawn(src, dest, all_moves, flag, true);
    }

    fn try_add_pawn(
        &self,
        src: usize,
        dest: i32,
        all_moves: &mut Vec<IntMove>,
        flag: usize,
        capture_ok: bool
    ) {
        if dest & 0x88 == 0 {
            assert!(dest >= 0);
            let (color, piece) = boardcell_destruct(self.board[src]);
            let opp_color = color.opposite();
            let dest_field = self.board[dest as usize];

            if dest_field == EMPTY || (capture_ok && boardcell_player(dest_field) == opp_color) {
                if piece != Piece::Pawn {
                    all_moves.push(intmove_encode_flags(src, dest as usize, None, flag));
                } else {
                    let reaches_last_row = match color {
                        Player::White => dest >= 112, //dest_row == 7,
                        Player::Black => dest <= 7, //dest_row == 0,
                    };

                    if reaches_last_row {
                        for &promo_piece in PROMOTABLE_PIECES.iter() {
                            all_moves.push(intmove_encode_flags(src, dest as usize, Some(promo_piece), flag));
                        }
                    } else {
                        all_moves.push(intmove_encode_flags(src, dest as usize, None, flag));
                    }
                }
            }
        }
    }

    fn generate_moves_from(&self, src: usize, piece: Piece, color: Player, all_moves: &mut Vec<IntMove>) {
        assert_eq!(boardcell_encode(color, piece), self.board[src]);

        let RowCol {
            row: src_row,
            col: _src_col,
        } = index2rowcol(src);

        let delta: i32 = if color == Player::White { 16 } else { -16 };

        match piece {
            Piece::Pawn => {
                let is_first_move = match color {
                    Player::White => src_row == 1,
                    Player::Black => src_row == 6,
                };

                let square_in_front = src as i32 + delta;

                self.try_add_pawn(src, square_in_front, all_moves, 0, false);

                // first move by two squares
                if is_first_move {
                    // make sure the square before the pawn is empty!
                    if self.board[square_in_front as usize] == EMPTY {
                        self.try_add_pawn(
                            src,
                            src as i32 + 2 * delta,
                            all_moves,
                            0,
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
                                0,
                                true,
                            );
                        }
                    }
                }
            }
            Piece::King => {
                for delta in &[ 16, 17, 1, -15, -16, -17, -1, 15] {
                    self.try_add(src, src as i32 + delta, all_moves);
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
                            self.try_add(src, f2 as i32, all_moves);
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
                            self.try_add_flag(src, f2 as i32, all_moves, CASTLE_FLAG);
                        }
                    }
                }
            }
            Piece::Knight => {
                for delta in &[ 31, 33, -31, -33, 18, -14, -18, 14] {
                    self.try_add(src, src as i32 + delta, all_moves);
                }
            }
            Piece::Queen => {
                let deltas = &[17, -15, -17, 15, 16, 1, -16, -1];
                self.line(src, deltas, all_moves);
            }
            Piece::Bishop => {
                let deltas = &[17, -15, -17, 15];
                self.line(src, deltas, all_moves);
            }
            Piece::Rook => {
                let deltas = &[16, 1, -16, -1];
                self.line(src, deltas, all_moves);
            }
        }
    }

    pub fn moves(&self) -> Vec<IntMove> {
        self.moves_by(self.to_move)
    }

    pub fn moves_by(&self, color: Player) -> Vec<IntMove> {
        let mut all_moves = vec![];

        // TODO: put the indexes in a table
        for index in 0..MAX_INDEX88 {
            if index & 0x88 == 0 {
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
            }

        all_moves
    }

    fn king_location(&self, player: Player) -> Option<usize> {
        let king = boardcell_encode(player, Piece::King);
        for index in 0..MAX_INDEX88 {
            if index & 0x88 == 0 {
                if self.board[index] == king {
                    return Some(index);
                }
            }
        }
        return None;
    }

    pub fn fields_attacked_by(&self, player: Player) -> Vec<usize> {
        // TODO: use a set here
        self.moves_by(player)
            .iter()
            .map(|mv: &IntMove| intmove_dest(*mv))
            .collect()
    }

    pub fn is_king_in_check(&self, player: Player) -> bool {
        let king_coord = self.king_location(player).unwrap();
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
        println!("Found {} pseudo-legal moves", moves.len());
        println!("Returning {} moves", result.len());
        result
    }

    pub fn is_checkmate(&mut self) -> bool {
        self.legal_moves().len() == 0 && self.is_king_in_check(self.to_move)
    }

    pub fn is_stalemate(&mut self) -> bool {
        self.legal_moves().len() == 0 && !self.is_king_in_check(self.to_move)
    }
}

#[cfg(test)]
mod test_moves {

    use super::*;

    #[test]
    fn moves_pawns_initial() {
        let fen = "8/8/8/8/8/8/PPPPPPPP/8 w KQkq - 0 1";
        let pos = Position::from_fen(fen);
        let moves = pos.moves();

        assert_eq!(16, moves.len());
    }

    #[test]
    fn moves_pawns_second() {
        let fen = "8/8/8/8/8/PPPPPPPP/8/8 w KQkq - 0 1";
        let pos = Position::from_fen(fen);
        let moves = pos.moves();

        assert_eq!(8, moves.len());
    }

    #[test]
    fn moves_pawns_promote_one() {
        let fen = "8/7P/8/8/8/8/8/8 w KQkq - 0 1";
        let pos = Position::from_fen(fen);
        let moves = pos.moves();
        println!("moves = {:#?}", moves);

        assert_eq!(4, moves.len());
    }

    #[test]
    fn moves_pawns_promote_full() {
        let fen = "8/PPPPPPPP/8/8/8/8/8/8 w KQkq - 0 1";
        let pos = Position::from_fen(fen);
        let moves = pos.moves();

        assert_eq!(8 * 4, moves.len());
    }

    #[test]
    fn moves_pawns_capture() {
        let fen = "8/8/p1p5/1P6/8/8/8/8 w KQkq - 0 1";
        let pos = Position::from_fen(fen);
        let moves = pos.moves();

        assert_eq!(3, moves.len());
    }

    #[test]
    fn moves_pawns_capture_and_promote() {
        let fen = "p1p5/1P6/8/8/8/8/8/8 w KQkq - 0 1";
        let pos = Position::from_fen(fen);
        let moves = pos.moves();

        assert_eq!(3 * 4, moves.len());
    }

    #[test]
    fn moves_pawns_no_capture_forward() {
        let fen = "8/8/1p6/1P6/8/8/8/8 w KQkq - 0 1";
        let pos = Position::from_fen(fen);
        let moves = pos.moves();

        assert_eq!(0, moves.len());
    }

    #[test]
    fn moves_pawns_en_passant() {
        let fen = "8/8/8/8/4Pp2/8/8/8 b - e3 0 1";
        let pos = Position::from_fen(fen);
        let moves = pos.moves();

        assert_eq!(2, moves.len());
    }

    #[test]
    fn moves_king_corners() {
        let fens = [
            "K7/8/8/8/8/8/8/8 w KQkq - 0 1",
            "7K/8/8/8/8/8/8/8 w KQkq - 0 1",
            "8/8/8/8/8/8/8/K7 w KQkq - 0 1",
            "8/8/8/8/8/8/8/7K w KQkq - 0 1",
        ];

        for &fen in fens.iter() {
            let pos = Position::from_fen(fen);
            let moves = pos.moves();
            println!("moves = {:#?}", moves);

            assert_eq!(3, moves.len());
        }
    }

    #[test]
    fn moves_king_edges() {
        let fens = [
            "3K4/8/8/8/8/8/8/8 w KQkq - 0 1",
            "8/8/8/K7/8/8/8/8 w KQkq - 0 1",
            "8/8/8/7K/8/8/8/8 w KQkq - 0 1",
            "8/8/8/8/8/8/8/5K2 w KQkq - 0 1",
        ];

        for &fen in fens.iter() {
            let pos = Position::from_fen(fen);
            let moves = pos.moves();
            println!("moves = {:#?}", moves);

            assert_eq!(5, moves.len());
        }
    }

    #[test]
    fn moves_king_inside() {
        let fens = [
            "8/3K4/8/8/8/8/8/8 w KQkq - 0 1",
            "8/8/8/5K2/8/8/8/8 w KQkq - 0 1",
            "8/8/8/8/8/8/1K6/8 w KQkq - 0 1",
            "8/8/8/8/8/4K3/8/8 w KQkq - 0 1",
        ];

        for &fen in fens.iter() {
            let pos = Position::from_fen(fen);
            let moves = pos.moves();
            println!("moves = {:#?}", moves);

            assert_eq!(8, moves.len());
        }
    }

    #[test]
    fn moves_knight_corners() {
        let fens = [
            "N7/8/8/8/8/8/8/8 w KQkq - 0 1",
            "7N/8/8/8/8/8/8/8 w KQkq - 0 1",
            "8/8/8/8/8/8/8/N7 w KQkq - 0 1",
            "8/8/8/8/8/8/8/7N w KQkq - 0 1",
        ];

        for &fen in fens.iter() {
            let pos = Position::from_fen(fen);
            let moves = pos.moves();
            println!("moves = {:#?}", moves);

            assert_eq!(2, moves.len());
        }
    }

    #[test]
    fn moves_knight_edges() {
        let fens = [
            "3N4/8/8/8/8/8/8/8 w KQkq - 0 1",
            "8/8/8/N7/8/8/8/8 w KQkq - 0 1",
            "8/8/8/7N/8/8/8/8 w KQkq - 0 1",
            "8/8/8/8/8/8/8/5N2 w KQkq - 0 1",
        ];

        for &fen in fens.iter() {
            let pos = Position::from_fen(fen);
            let moves = pos.moves();

            println!("{}", pos.to_ascii());
            println!("moves = {:#?}", moves);

            for &mv in &moves {
                println!("mv = {}", intmove_to_uci_ascii(mv));
            }

            assert_eq!(4, moves.len());
        }
    }

    #[test]
    fn moves_knight_one_step_from_edge() {
        let fens = [
            "8/8/1N6/8/8/8/8/8 w KQkq - 0 1",
            "8/8/8/8/8/6N1/8/8 w KQkq - 0 1",
            "8/8/8/8/8/1N6/8/8 w KQkq - 0 1",
            "8/8/8/8/6N1/8/8/8 w KQkq - 0 1",
        ];

        for &fen in fens.iter() {
            let pos = Position::from_fen(fen);
            let moves = pos.moves();
            println!("moves = {:#?}", moves);

            assert_eq!(6, moves.len());
        }
    }

    #[test]
    fn moves_knight_inside() {
        let fens = [
            "8/8/2N5/8/8/8/8/8 w KQkq - 0 1",
            "8/8/8/8/8/4N3/8/8 w KQkq - 0 1",
            "8/8/8/8/8/3N4/8/8 w KQkq - 0 1",
            "8/8/8/8/5N2/8/8/8 w KQkq - 0 1",
        ];

        for &fen in fens.iter() {
            let pos = Position::from_fen(fen);
            let moves = pos.moves();
            println!("moves = {:#?}", moves);

            assert_eq!(8, moves.len());
        }
    }

    #[test]
    fn moves_rook() {
        let fens = [
            "R7/8/8/8/8/8/8/8 w KQkq - 0 1",
            "4R3/8/8/8/8/8/8/8 w KQkq - 0 1",
            "7R/8/8/8/8/8/8/8 w KQkq - 0 1",
            "8/8/R7/8/8/8/8/8 w KQkq - 0 1",
            "8/8/4R3/8/8/8/8/8 w KQkq - 0 1",
            "8/8/7R/8/8/8/8/8 w KQkq - 0 1",
            "8/8/8/8/8/8/8/R7 w KQkq - 0 1",
            "8/8/8/8/8/8/8/4R3 w KQkq - 0 1",
            "8/8/8/8/8/8/8/7R w KQkq - 0 1",
        ];

        for &fen in fens.iter() {
            let pos = Position::from_fen(fen);
            let moves = pos.moves();
            println!("moves = {:#?}", moves);

            assert_eq!(14, moves.len());
        }
    }

    #[test]
    fn moves_queen_corner() {
        let fens = [
            "Q7/8/8/8/8/8/8/8 w KQkq - 0 1",
            "7Q/8/8/8/8/8/8/8 w KQkq - 0 1",
            "8/8/8/8/8/8/8/Q7 w KQkq - 0 1",
            "8/8/8/8/8/8/8/7Q w KQkq - 0 1",
        ];

        for &fen in fens.iter() {
            let pos = Position::from_fen(fen);
            let moves = pos.moves();
            println!("moves = {:#?}", moves);

            assert_eq!(21, moves.len());
        }
    }

    #[test]
    fn moves_queen_inside() {
        let fens = [
            "8/8/8/8/4Q3/8/8/8 w KQkq - 0 1",
            "8/8/8/3Q4/8/8/8/8 w KQkq - 0 1",
        ];

        for &fen in fens.iter() {
            let pos = Position::from_fen(fen);
            let moves = pos.moves();
            println!("moves = {:#?}", moves);

            assert_eq!(27, moves.len());
        }
    }

    #[test]
    fn moves_queen_inside2() {
        let fens = [
            "8/8/8/8/8/5Q2/8/8 w KQkq - 0 1",
            "8/8/2Q5/8/8/8/8/8 w KQkq - 0 1",
        ];

        for &fen in fens.iter() {
            let pos = Position::from_fen(fen);
            let moves = pos.moves();
            println!("moves = {:#?}", moves);

            assert_eq!(25, moves.len());
        }
    }

    #[test]
    fn moves_queen_edges() {
        let fens = [
            "3Q4/8/8/8/8/8/8/8 w KQkq - 0 1",
            "8/8/8/Q7/8/8/8/8 w KQkq - 0 1",
            "8/8/8/7Q/8/8/8/8 w KQkq - 0 1",
            "8/8/8/8/8/8/8/5Q2 w KQkq - 0 1",
        ];

        for &fen in fens.iter() {
            let pos = Position::from_fen(fen);
            let moves = pos.moves();
            println!("moves = {:#?}", moves);

            assert_eq!(21, moves.len());
        }
    }

    #[test]
    fn moves_queen_blocked() {
        let inputs = [
            ("Qp6/pp6/8/8/8/8/8/8 w KQkq - 0 1", 3),
            (
                "rnbqkbnr/pppppppp/8/8/8/3P4/PPP1PPPP/RNBQKBNR w KQkq - 0 1",
                27,
            ),
            (
                "rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 1",
                28,
            ),
        ];

        for (fen, value) in inputs.iter() {
            let pos = Position::from_fen(fen);
            let moves = pos.moves();
            println!("{}", pos.to_ascii());
            println!("moves = {:#?}", moves);
            for &mv in &moves {
                println!("mv = {}", intmove_to_uci_ascii(mv));
            }

            assert_eq!(*value, moves.len());
        }
    }

    #[test]
    fn moves_bishop_corner() {
        let fens = [
            "B7/8/8/8/8/8/8/8 w KQkq - 0 1",
            "7B/8/8/8/8/8/8/8 w KQkq - 0 1",
            "8/8/8/8/8/8/8/B7 w KQkq - 0 1",
            "8/8/8/8/8/8/8/7B w KQkq - 0 1",
        ];

        for &fen in fens.iter() {
            let pos = Position::from_fen(fen);
            let moves = pos.moves();
            println!("moves = {:#?}", moves);

            assert_eq!(7, moves.len());
        }
    }

    #[test]
    fn moves_bishop_edges() {
        let fens = [
            "3B4/8/8/8/8/8/8/8 w KQkq - 0 1",
            "8/8/8/B7/8/8/8/8 w KQkq - 0 1",
            "8/8/8/7B/8/8/8/8 w KQkq - 0 1",
            "8/8/8/8/8/8/8/5B2 w KQkq - 0 1",
        ];

        for &fen in fens.iter() {
            let pos = Position::from_fen(fen);
            let moves = pos.moves();
            println!("moves = {:#?}", moves);

            assert_eq!(7, moves.len());
        }
    }

    #[test]
    fn moves_bishop_inside() {
        let fens = [
            "8/8/8/8/4B3/8/8/8 w KQkq - 0 1",
            "8/8/8/3B4/8/8/8/8 w KQkq - 0 1",
        ];

        for &fen in fens.iter() {
            let pos = Position::from_fen(fen);
            let moves = pos.moves();
            println!("moves = {:#?}", moves);

            assert_eq!(13, moves.len());
        }
    }

    #[test]
    fn moves_bishop_inside2() {
        let fens = [
            "8/8/8/8/8/5B2/8/8 w KQkq - 0 1",
            "8/8/2B5/8/8/8/8/8 w KQkq - 0 1",
        ];

        for &fen in fens.iter() {
            let pos = Position::from_fen(fen);
            let moves = pos.moves();
            println!("moves = {:#?}", moves);

            assert_eq!(11, moves.len());
        }
    }

    #[test]
    fn not_in_check() {
        let player = board::Player::White;
        let fens = [
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            "8/8/8/8/8/8/8/4K3 w KQkq - 0 1",
        ];

        for &fen in fens.iter() {
            let pos = Position::from_fen(fen);
            assert_eq!(false, pos.is_king_in_check(player));
        }
    }

    #[test]
    fn in_check() {
        let player = board::Player::White;
        let fens = [
            "8/8/8/8/8/8/8/r3K3 w KQkq - 0 1",
            "7b/8/8/8/8/8/8/K7 w KQkq - 0 1",
            "8/8/8/8/8/8/1p6/K7 w KQkq - 0 1",
            "8/8/8/8/8/1n6/8/K7 w KQkq - 0 1",
        ];

        for &fen in fens.iter() {
            let pos = Position::from_fen(fen);
            assert_eq!(true, pos.is_king_in_check(player));
        }
    }

    #[test]
    fn filter_moves_leaving_king_in_check() {
        let inputs = [(
            "rnbqkbnr/ppppp1pp/5p2/7Q/4P3/8/PPPP1PPP/RNB1KBNR b KQkq - 1 2",
            1,
        )];

        for (fen, value) in inputs.iter() {
            let mut pos = Position::from_fen(fen);
            let moves = pos.legal_moves();
            println!("moves = {:#?}", moves);

            assert_eq!(*value, moves.len());
        }
    }

    #[test]
    fn castling_moves() {
        let inputs = [
            (
                // king side
                "4k3/8/8/8/8/8/7P/4K2R w K - 0 1",
                10,
            ),
            (
                // queen side
                "4k3/8/8/8/8/8/P7/R3K3 w Q - 0 1",
                11,
            ),
            (
                // italian game example
                "r1bqk1nr/pppp1ppp/2n5/2b1p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 4 4",
                33,
            ),
        ];

        for (fen, value) in inputs.iter() {
            let mut pos = Position::from_fen(fen);
            let moves = pos.legal_moves();
            println!("moves = {:#?}", moves);

            assert_eq!(*value, moves.len());
        }
    }

    #[test]
    fn castling_impossible_no_rights() {
        let inputs = [
            (
                // king side
                "4k3/8/8/8/8/8/7P/4K2R w - - 0 1",
                9,
            ),
            (
                // queen side
                "4k3/8/8/8/8/8/P7/R3K3 w - - 0 1",
                10,
            ),
            (
                // italian game example
                "r1bqk1nr/pppp1ppp/2n5/2b1p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w - - 4 4",
                32,
            ),
        ];

        for (fen, value) in inputs.iter() {
            let mut pos = Position::from_fen(fen);
            let moves = pos.legal_moves();
            println!("moves = {:#?}", moves);

            assert_eq!(*value, moves.len());
        }
    }

    #[test]
    fn castling_impossible_other() {
        let inputs = [
            (
                // king in check
                "1k6/8/8/8/4r3/8/8/R3K2R w KQ - 0 1",
                4,
            ),
            (
                // squares occupied
                "1k6/2r5/8/8/8/8/P6P/RN2K1NR w KQ - 0 1",
                15,
            ),
            (
                // unsafe square
                "k7/8/8/8/3r1r2/8/7P/4K2R w K - 0 1",
                5,
            ),
            (
                // unsafe square
                "k7/8/8/8/3r2r1/8/r6P/4K2R w K - 0 1",
                5,
            ),
            (
                // unsafe square
                "k7/8/8/8/3r2r1/8/P6r/R3K3 w Q - 0 1",
                6,
            ),
            (
                // unsafe square
                "k7/8/8/8/2r3r1/8/P6r/R3K3 w Q - 0 1",
                7,
            ),
        ];

        for (fen, value) in inputs.iter() {
            let mut pos = Position::from_fen(fen);
            let moves = pos.legal_moves();
            println!("moves = {:#?}", moves);

            assert_eq!(*value, moves.len());
        }
    }
}
