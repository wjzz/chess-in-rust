# Chess-in-rust [![Build Status](https://travis-ci.org/wjzz/chess-in-rust.svg?branch=master)](https://travis-ci.org/wjzz/chess-in-rust)

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
- [ ] `stalemate` checking
- [x] legal move generation

PERFT [clone instead of unmove]: (bugs found)
- [x] depth = 1
- [x] depth = 2    # color change after move
- [x] depth = 3    # pawn captured forward, pawn jumping over knight
- [x] depth = 4    # king cant be left in check
- [ ] depth = 5    TODO: legal_moves is rather slow, en passant is missing

Later:
- [ ] Bitboards
- [ ] Experiments with more efficient/compact data structures
- [ ] Compilation to WASM
- [ ] Frontend in JS

Other/infra:
- [x] Put the repo under Travis/CI
- [ ] Some tests can be slow on `debug` build, consider reading about bechmarks for cargo

Wojciech Jedynak @ 2020
