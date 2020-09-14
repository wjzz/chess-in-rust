use std::sync::{Arc, Mutex};
use std::thread;

#[path = "make_move.rs"]
pub mod make_move;

pub use make_move::*;

impl Position {
    fn perft_immutable_iter(depth: u32, level: u32, pos: Position) -> u32 {
        if depth == 0 {
            return 1;
        }

        let moves = pos.legal_moves();
        let mut result = 0;

        if depth == 1 {
            return moves.len() as u32;
        }

        for &mv in moves.iter() {
            // if level == 0 && depth > 4 {
            //     println!(" {:2}/{}\t{}->{}", counter, moves.len(), mv.src, mv.dest);
            //     counter += 1;
            // }
            let mut pos2 = pos.clone();
            pos2.make_move(mv).unwrap();
            result += Position::perft_immutable_iter(depth - 1, level + 1, pos2);
        }

        result
    }

    pub fn perft_immutable(depth: u32, fen: &str) -> u32 {
        let pos = Position::from_fen(fen);
        Position::perft_immutable_iter(depth, 0, pos)
    }

    pub fn perft_immutable_par(depth: u32, fen: &str, n_threads: u32) -> u32 {
        let pos = Position::from_fen(fen);

        let moves = pos.legal_moves();

        let mtx = Arc::new(Mutex::new(0));
        let mut threads = vec![];

        for id in 0..n_threads {
            let mtx2 = Arc::clone(&mtx);

            let moves = moves.clone();
            let pos = pos.clone();

            threads.push(thread::spawn(move || {
                let mut i = id as usize;

                while i < moves.len() {
                    let mv = moves[i];

                    let start = std::time::Instant::now();

                    let mut pos2 = pos.clone();
                    pos2.make_move(mv).unwrap();

                    let value = Position::perft_immutable_iter(depth - 1, 1, pos2);
                    let mut result = mtx2.lock().unwrap();
                    *result += value;

                    if depth > 5 {
                        println!(
                            " Thread {} finished mv {:2}/{}\t{}->{} after {:.2?}",
                            id + 1,
                            i + 1,
                            moves.len(),
                            index2coord(mv.src),
                            index2coord(mv.dest),
                            start.elapsed()
                        );
                    }

                    i += n_threads as usize;
                }
            }));
        }

        for t in threads {
            t.join().unwrap();
        }

        let x = *mtx.lock().unwrap();
        x
    }
}
