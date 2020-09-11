use rust_chess::Position;

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line = line.trim_end().to_string();

    let pos = Position::from_fen(&line);
    println!("{}", pos.to_ascii());
    println!();

    let moves = pos.moves();
    println!("{:#?}", moves);
    println!("Total {} moves", moves.len());
}