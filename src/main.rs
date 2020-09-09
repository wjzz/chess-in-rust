mod board;
mod parser;

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line = line.trim_end().to_string();

    let pos = parser::parse_fen(&line);
    println!("{}", pos.to_ascii());
}
