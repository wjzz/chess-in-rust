# Chess-in-rust [![Build Status](https://travis-ci.org/wjzz/chess-in-rust.svg?branch=master)](https://travis-ci.org/wjzz/chess-in-rust)

## A simple chess engine written in Rust

Priority TODOs:

- [x] Basic data structures
- [x] Basic move generation
- - [x] En passant
- - [ ] Castling
- [x] Make move
- [ ] Unmake move
- [x] `check` checking
- [ ] `mate` checking
- [ ] `stalemate` checking
- [x] legal move generation
- [x] half_move counter since pawn move or capture for fen
- [x] full_move counter for fen
- [ ] implement faster board indexing, don't use `Coord`s everywhere

PERFT [clone instead of unmove]: (bugs found)
- [x] depth = 1
- [x] depth = 2    # color change after move
- [x] depth = 3    # pawn captured forward, pawn jumping over knight
- [x] depth = 4    # king cant be left in check
- [x] depth = 5    # missing en passant    TODO: requires more than 3 minutes, need to optimize
- [x] depth = 6    # TODO: requires 60min on single core | 20 min on all cores
- [ ] depth = 7    # no result after 6h hours

Later:
- [ ] Bitboards
- [ ] Experiments with more efficient/compact data structures
- [ ] Compilation to WASM
- [ ] Frontend in JS

Other/infra:
- [x] Put the repo under Travis/CI
- [ ] Some tests can be slow on `debug` build, consider reading about bechmarks for cargo

## To run slow tests

`cargo test -- --ignored`

## Credits

Wojciech Jedynak @ 2020
