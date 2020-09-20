# Chess-in-rust [![Build Status](https://travis-ci.org/wjzz/chess-in-rust.svg?branch=master)](https://travis-ci.org/wjzz/chess-in-rust)

## A simple chess engine written in Rust

Priority TODOs:

- [x] Basic data structures
- [x] Basic move generation
- - [x] En passant
- - [x] Castling
- [x] Make move
- [x] Unmake move
- [x] `check` checking
- [x] `mate` checking
- [x] `stalemate` checking
- [x] legal move generation
- [x] half_move counter since pawn move or capture for fen
- [x] full_move counter for fen
- [x] implement faster board indexing, don't use `Coord`s everywhere
- [ ] `draw by insufficent material` checking
- [ ] draw by repetition
- [ ] position hash
- [ ] king in check flag

Bot TODOs:

- [x] alpha-beta
- [x] opening book (using python's driver and a book found somewhere)
- [ ] ending table

PERFT [clone instead of unmove]: (bugs found)
- [x] depth = 1
- [x] depth = 2    # color change after move
- [x] depth = 3    # pawn captured forward, pawn jumping over knight
- [x] depth = 4    # king cant be left in check
- [x] depth = 5    # missing en passant    TODO: requires more than 2 minutes, need to optimize
- [x] depth = 6    # TODO: requires 60min on single core | 20 min on all cores
- [x] depth = 7    # a few hours
- [ ] depth = 8

Later:
- [ ] Ox88 board
- [ ] Bitboards
- [ ] Experiments with more efficient/compact data structures
- - [ ] Use `usize` for repr. moves instead of three ints
- [ ] Compilation to WASM
- [ ] Frontend in JS

Other/infra:
- [x] Put the repo under Travis/CI
- [ ] Some tests can be slow on `debug` build, consider reading about bechmarks for cargo

## To run slow tests

`cargo test -- --ignored`

## Credits

Wojciech Jedynak @ 2020
