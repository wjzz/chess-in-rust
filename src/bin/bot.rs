use rust_chess::move_search::*;
use rust_chess::position::*;

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

    let mv = best_move_negamax(&mut pos, 3);
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
