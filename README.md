# Chess-in-rust

## A simple chess engine written in Rust

Priority TODOs:

- [x] Basic data structures
- [x] Basic move generation
- - [ ] En passant
- - [ ] Castling
- [x] Make move
- [ ] Unmake move
- [x] `check` checking
- [ ] `mate` checking
- [ ] legal move generation

PERFT [clone instead of unmove]: (bugs found)
- [x] depth = 1
- [x] depth = 2    # color change after move
- [x] depth = 3    # pawn captured forward, pawn jumping over knight
- [ ] depth = 4    # TODO: king cant be left in check

Later:
- [ ] Bitboards
- [ ] Experiments with more efficient/compact data structures
- [ ] Compilation to WASM
- [ ] Frontend in JS

Other/infra:
- [x] Put the repo under Travis/CI

Wojciech Jedynak @ 2020
