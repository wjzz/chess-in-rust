use super::super::super::position::*;

//-------------------------------------------------------
// MATERIAL BALANCE
//-------------------------------------------------------

const MATERIAL_VAL: [f64; 13] = [
     0.0 , // B_KING
    -9.0, // B_QUEEN
    -5.0, // B_ROOK
    -3.5, // B_BISHOP
    -3.0, // B_KNIGHT
    -1.0, // B_PAWN
    0.0, // EMPTY
    1.0, // W_PAWN
    3.0, // W_KNIGHT
    3.5, // W_BISHOP
    5.0, // W_ROOK
    9.0, // W_QUEEN
    0.0, // W_KING
];

fn eval_material(pos: &Position) -> f64 {
    let mut ev = 0.0;
    for i in INDEXES88.iter() {
        ev += MATERIAL_VAL[(pos.board[*i] + 6) as usize];
    }
    if pos.to_move == Player::White { ev } else { -ev }
}

//-------------------------------------------------------
// PIECE_POSITIONING
//-------------------------------------------------------

// B_KING
// B_QUEEN
// B_ROOK
// B_BISHOP
// B_KNIGHT
// B_PAWN
// EMPTY
// W_PAWN
// W_KNIGHT
// W_BISHOP
// W_ROOK
// W_QUEEN
// W_KING

const POS: [[i32; 64]; 6] = [
    // pawn
    [
          0,   0,   0,   0,   0,   0,   0,   0,
          0,   0,   0,   0,   0,   0,   0,   0,
          0,   0,   0,   0,   0,   0,   0,   0,
          0,   0,   0,   0,   0,   0,   0,   0,
          0,   0,   0,  20,  20,   0,   0,   0,
          0,   0,   0,   5,   5,   0,   0,   0,
          5,   5,   0, -10, -10,   0,   5,   5,
          0,   0,   0,   0,   0,   0,   0,   0,
    ],
    // knight
    [
        -20,   0,   0,   0,   0,   0,   0, -20,
          0,   0,   0,   0,   0,   0,   0,   0,
          0,   0,   0,   0,   0,   0,   0,   0,
          0,   0,   0,  45,  45,   0,   0,   0,
          0,   0,   0,  45,  45,   0,   0,   0,
          0,   0,  20,   0,   0,  20,   0,   0,
          0,   0,   0,   0,   0,   0,   0,   0,
        -20, -10,   0,   0,   0,   0, -10, -20,
    ],
    // bishop
    [
          0,   0,   0,   0,   0,   0,   0,   0,
          0,   0,   0,   0,   0,   0,   0,   0,
          0,   0,   0,   0,   0,   0,   0,   0,
          0,   0,   0,   5,   5,   0,   0,   0,
          0,   0,   0,  15,  15,   0,   0,   0,
          0,   0,   0,   5,   5,   0,   0,   0,
          0,   0,   0,   0,   0,   0,   0,   0,
          0,  -5,   0,   0,   0,   0,  -5,   0,
    ],
    // rook
    [
          0,   0,   0,   0,   0,   0,   0,   0,
         20,  30,  40,  50,  50,  40,  30,  20,
          0,   0,   0,   0,   0,   0,   0,   0,
          0,   0,   0,   0,   0,   0,   0,   0,
          0,   0,   0,   0,   0,   0,   0,   0,
          0,   0,   0,   0,   0,   0,   0,   0,
          0,   0,   0,   0,   0,   0,   0,   0,
          0,   0,   0,  20,  20,  10,   0,   0,
    ],
    // queen
    [
          0,   0,   0,   0,   0,   0,   0,   0,
          0,   0,   0,   0,   0,   0,   0,   0,
          0,   0,   0,   0,   0,   0,   0,   0,
          0,   0,   0,   5,   5,   0,   0,   0,
          0,   0,   0,   5,   5,   0,   0,   0,
          0,   0,   0,   0,   0,   0,   0,   0,
          0,   0,   0,   0,   0,   0,   0,   0,
          0,   0,   0,   0,   0,   0,   0,   0,
    ],
    // king
    [
          0,   0,   0,   0,   0,   0,   0,   0,
          0,   0,   0,   0,   0,   0,   0,   0,
          0,   0,   0,   0,   0,   0,   0,   0,
          0,   0,   0,   0,   0,   0,   0,   0,
          0,   0,   0,   0,   0,   0,   0,   0,
          0,   0,   0,   0,   0,   0,   0,   0,
          0,   0,   0,   0,   0,   0,   0,   0,
          0,  30,  25,   0, -10,   0,  30,   0,
    ],
];

fn eval_positioning(pos: &Position) -> i32 {
    let mut ev = 0;
    for &i in INDEXES88.iter() {
        let field = pos.board[i];
        if field != EMPTY {
            let i0x88 = if field > 0 { i } else { 119 - i };
            let i88 = (i0x88 + (i0x88 & 7)) >> 1;
            ev += (field / field.abs()) * POS[(field.abs()-1) as usize][i88];
        }
        // ev += MATERIAL_VAL[(pos.board[*i] + 6) as usize];
    }
    // ev *= 0.1;
    if pos.to_move == Player::White { ev } else { -ev }
}


//-------------------------------------------------------
// COMPOSITION
//-------------------------------------------------------

impl Position {
    pub fn eval(&self) -> f64 {
        let eval_pos = 0.01 * eval_positioning(&self) as f64;
        // if eval_pos != 0.0 {
        //     println!("eval pos = {}", eval_pos);
        // }
        return eval_material(&self) + eval_pos;
    }
}

