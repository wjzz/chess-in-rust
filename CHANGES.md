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