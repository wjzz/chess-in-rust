use rust_chess::position::*;

use rand::thread_rng;
use rand::seq::SliceRandom;

fn eval_piece(piece: Piece) -> f64 {
    match piece {
        Piece::Pawn => 1.0,
        Piece::Knight => 3.0,
        Piece::Bishop => 3.5,
        Piece::Rook => 5.0,
        Piece::Queen => 9.0,
        Piece::King => 0.0,
    }
}

enum EvalResult {
    Checkmate,
    Stalemate,
    Unfinished(f64),
}

fn eval_position(pos: &Position) -> f64 {
    let mut result = 0.0;
    for field in pos.board.iter() {
        match field {
            None => (),
            Some(PlayerPiece { player, piece }) => {
                let val = eval_piece(*piece);
                let sign = if pos.to_move == *player { 1.0 } else { -1.0 };
                result += val * sign;
            }
        }
    }
    result
}

fn negamax(pos: &Position, depth: i32) -> f64 {
    if pos.is_checkmate() {
        return -10000.0;
    } else if pos.is_stalemate() {
        return 0.0;
    }
    if depth == 0 {
        return eval_position(&pos);
    }

    let moves = pos.legal_moves();
    let mut best = -10000000.0;
    for &mv in moves.iter() {
        let mut pos2 = pos.clone();
        pos2.make_move(mv).unwrap();
        let val = -negamax(&pos2, depth-1);
        if val > best {
            best = val;
        }
    }
    best
}

fn best_move_negamax(pos: &Position) -> Move {
    let moves = pos.legal_moves();
    let mut best = 0.0;
    let mut best_index = 0;
    for (index, mv) in moves.iter().enumerate() {
        let mut pos2 = pos.clone();
        pos2.make_move(*mv).unwrap();
        let val = -negamax(&pos2, 2);
        if val > best || index == 0 {
            best = val;
            best_index = index;
        }
    }
    moves[best_index]
}

fn can_give_mate(pos: &Position) -> bool {
    for mv in pos.legal_moves() {
        let mut pos2 = pos.clone();
        pos2.make_move(mv).unwrap();
        if pos2.is_checkmate() {
            return true;
        }
    }
    return false;
}

fn choose_move(pos: &Position) -> Move {
    let mut rng = thread_rng();

    let mut good_moves = vec![];

    let moves = pos.legal_moves();
    for &mv in moves.iter() {
        let mut pos2 = pos.clone();
        pos2.make_move(mv).unwrap();
        if !can_give_mate(&pos2) {
            good_moves.push(mv);
        }
    }

    if good_moves.len() > 0 {
        return *good_moves.choose(&mut rng).unwrap();
    } else {
        return *moves.choose(&mut rng).unwrap();
    }
}

fn choose_move_rng(pos: &Position) -> Move {
    let mut rng = thread_rng();

    let moves = pos.legal_moves();
    return *moves.choose(&mut rng).unwrap();
    // return moves[0];
}

fn play_move(fen: &str, moves: &[&str]) -> String {
    let fen = if fen == "startpos" {
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    } else {
        fen
    };

    let mut pos = Position::from_fen(fen);
    for mv_str in moves.iter() {
        let mv = Move::from_uci_ascii(mv_str);
        pos.make_move(mv).unwrap();
    }

    let mv = best_move_negamax(&pos);
    // let mv = choose_move(&pos);
    mv.to_usi_ascii()
}

fn read_line() -> String {
    let mut buf = String::new();
    std::io::stdin()
        .read_line(&mut buf)
        .expect("Can't get input");
    buf = String::from(buf.trim());
    buf
}

fn main() {
    let _first_line = read_line();
    println!("id name rust-chess");
    println!("id author Wojciech Jedynak");
    println!("usiok");
    loop {
        let line = read_line();

        if line == "isready" {
            println!("readyok");
        } else if line == "quit" {
            break;
        } else if line == "stop" {
            break;
        } else if line.starts_with("position") {
            let parts: Vec<_> = line.split_ascii_whitespace().collect();
            // println!("<< parts1 {:?}", parts);
            if parts[0] == "position" {
                let fen = parts[1];
                let moves = if parts.len() > 2 {
                    let moves_str = parts[2];
                    assert_eq!(moves_str, "moves");
                    &parts[3..]
                } else {
                    &[]
                };

                let line2 = read_line();
                let parts2: Vec<_> = line2.split_ascii_whitespace().collect();
                // println!("<< parts2 {:?}", parts2);

                assert_eq!(parts2[0], "go");
                let bot_move = play_move(fen, moves);
                println!("bestmove {}", bot_move);
            }
        } else {
            if line != "" {
                eprintln!("Unknown command: {}", line);
            }
        }
    }
}

//position startpos moves e2e4
//go wtime 300000 btime 300000 winc 0 binc 0
