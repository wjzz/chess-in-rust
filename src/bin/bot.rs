use rust_chess::move_search::*;
use rust_chess::position::*;

fn play_move(fen: &str, moves: &[&str], wtime: i32, btime: i32, table: &TABLE) -> String {
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

    if let Some(&(res, depth, mv)) = table.get(&pos.hash) {
        println!("Found position in the endgame database. After {} moves the result is {}", depth, res);
        return intmove_to_uci_ascii(mv);
    }
    // eprintln!();

    let time_left = if pos.to_move == Player::White { wtime } else { btime };
    let final_fen = pos.to_fen();

    eprintln!("FEN: {}", final_fen);
    let mv = if time_left > 60 * 1000 {
        let (mv, _) = alphabeta_iterative_deepening(&mut pos, 5, true);
        mv
    } else if wtime == 0 && btime == 0 {
        alphabeta_iterative_deepening_infinite(&mut pos, true);
        panic!("This is infinite!")
        // infinite
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

    let mut table = endgame::initialize_table();
    endgame::generate_endgame_table(&mut table);

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
                if parts2[1] == "movetime" {
                    let time: i32 = parts2[2].parse().unwrap();
                    (time, time)
                } else {
                    assert_eq!(parts2[1], "infinite");
                    (0, 0)
                }
            };
            let bot_move = play_move(&fen, moves, wtime, btime, &table);
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
