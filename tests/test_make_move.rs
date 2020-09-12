use rust_chess::*;

#[cfg(test)]
mod test_moves {

    use super::*;

    #[test]
    fn make_move_starting() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

        let moves = [
            // pawns
            "A2->A3",
            "A2->A4",
            "B2->B3",
            "B2->B4",
            "C2->C3",
            "C2->C4",
            "D2->D3",
            "D2->D4",
            "E2->E3",
            "E2->E4",
            "F2->F3",
            "F2->F4",
            "G2->G3",
            "G2->G4",
            "H2->H3",
            "H2->H4",
            // knights
            "B1->A3",
            "B1->C3",
            "G1->F3",
            "G1->H3",
        ];

        for mv in moves.iter() {
            let pos = Position::from_fen(fen);
            let mv = Move::from_ascii(mv);
            // println!("Move: {:?}", mv);
        }
    }
}