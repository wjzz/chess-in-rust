# 2020-09-19

- Implemented `unmake`. After profiling and removing some debugging code, got a speedup to PERFT 5 from `3.2s` to `2.1s`
- Implemented `alpha-beta` and `PVS` this allowed us to win against stockfish lvl 1 and 2. lvl 3 is way too strong - we must
search deeper to win.

# 2020-09-20

- Tried to deepen PERFT by using a simple TT (HashMap<String, u32>) but it didn't change the result at all. I used `to_fen` as a hashing function, and it probably made it worse.
- Added `IntMove` which encodes a move in a single 64 `usize` word. This gives about 10% speedup and we have space for more flags.
- Tried to add a `CASTLE_FLAG` to the move repr, but there were small node difference in `bench` so I stashed the changes.
- Changed the Field type into a normal int with 0 == empty. No big performance changes, but lot of potential
- Found many vector copying in move_generation. Changed function to pass a ref to vector and gained `20%` speedup!
- Changed the representation of `en passant` field from coord to index. This allowed me to remove many coord2index calls. Got 5% speedup.
- Changed the board representation to 0x88. Wrote 0xff so everything was broken. Also for i in 0..64 tripped me a few time.

# 2020-09-21

- Perft starting at depth 8 run for 7 hours but wasn't correct by a small number. Added more perfts to the test suite.
- Fixed a castling check bug: a pawn's diagonal attack is not degenerated.
- Implemented a quicker `is_king_in_check` procedure that checks the position of the king and checks if it is acheivable for the current piece. This uses some nice direction uniqueness properties of the `0x88` repr. The `speedup gain is huge` - 66%, the program runs 3x quicker!!
- Tried to play against stockfish lvl 3 again, but we get easily outplayed even if we get an early advantage.
- Calculated perft lvl 8 for startpos!

# 2020-09-22

- Found a big mistake in negamax et al - `legal_moves` were generated thrice per node - for checkmate, stalemate and finally for moves!
- Remove the old `is_king_in_check` and replace it by `is_king_in_check_fast`
- The engine can't mate a naked king with rook and pawn :-)