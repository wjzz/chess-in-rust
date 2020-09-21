use rust_chess::move_search::*;
use rust_chess::position::*;

fn play_move(fen: &str, moves: &[&str], wtime: i32, btime: i32) -> String {
    let fen = if fen == "startpos" {
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    } else {
        fen
    };

    let mut pos = Position::from_fen(fen);

    for mv_str in moves.iter() {
        let mv = Move::from_uci_ascii(mv_str);
        let mv = intmove_from_move(&mv);
        // eprint!("{} ", intmove_to_uci_ascii(mv));
        pos.make_move(mv).unwrap();
    }
    // eprintln!();

    let time_left = if pos.to_move == Player::White { wtime } else { btime };
    let final_fen = pos.to_fen();

    eprintln!("FEN: {}", final_fen);
    let mv = if time_left > 60 * 1000 {
        let mv = best_move_iterative_deepening(&mut pos, 4);
        // let (mv, val) = best_move_pvs(&mut pos, 5);
        // let mv = best_move_negamax(&mut pos, 3);
        // let mv = choose_move(&pos);
        // eprintln!("MOVE: {} EVAL: {:.1}", intmove_to_uci_ascii(mv), val);
        mv
    } else {
        let mv = choose_move(&mut pos);
        // eprintln!("MOVE: {}", intmove_to_uci_ascii(mv));
        mv
    };

    intmove_to_uci_ascii(mv)
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

        // if line != "" {
        //     eprintln!("<< {}", line);
        // }

        if line == "isready" {
            println!("readyok");
        } else if line == "quit" {
            break;
        } else if line == "stop" {
            break;
        } else if line.starts_with("position") {
            let fen;// = String::new();
            let mut moves: &[&str] = &[];

            let parts: Vec<_> = line.split_ascii_whitespace().collect();
            if line.starts_with("position fen") {
                let fen_parts = &parts[2..8];
                fen = fen_parts.join(" ");
                if parts.len() > 8 {
                    assert_eq!(parts[8], "moves");
                    moves = &parts[9..];
                }
            } else {
                assert_eq!(parts[0], "position");
                assert_eq!(parts[1], "startpos");

                fen = String::from("startpos");

                if parts.len() > 2 {
                    let moves_str = parts[2];
                    assert_eq!(moves_str, "moves");
                    moves = &parts[3..]
                }
            }

            // eprintln!("FEN IS = {}", fen);

            let line2 = read_line();
            let parts2: Vec<_> = line2.split_ascii_whitespace().collect();
            assert_eq!(parts2[0], "go");
            let (wtime, btime) = if parts2[1] == "wtime" {
                let wtime: i32 = parts2[2].parse().unwrap();
                assert_eq!(parts2[3], "btime");
                let btime: i32 = parts2[4].parse().unwrap();
                (wtime, btime)
            } else {
                assert_eq!(parts2[1], "movetime");
                let time: i32 = parts2[2].parse().unwrap();
                (time, time)
            };
            let bot_move = play_move(&fen, moves, wtime, btime);
            println!("bestmove {}", bot_move);
        } else {
            if line != "" {
                // eprintln!("Unknown command: {}", line);
            }
        }
    }
}

//position startpos moves e2e4
//go wtime 300000 btime 300000 winc 0 binc 0
