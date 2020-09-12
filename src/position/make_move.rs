#[path = "move_gen.rs"]
pub mod move_gen;

pub use move_gen::*;

impl Position {
    pub fn make_move(&mut self, &mv: Move) -> Result<(), String> {
        unimplemented!()
    }
}