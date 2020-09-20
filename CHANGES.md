# 2020-09-19

- Implemented `unmake`. After profiling and removing some debugging code, got a speedup to PERFT 5 from `3.2s` to `2.1s`
- Implemented `alpha-beta` and `PVS` this allowed us to win against stockfish lvl 1 and 2. lvl 3 is way too strong - we must
search deeper to win.

# 2020-09-20

- Tried to deepen PERFT by using a simple TT (HashMap<String, u32>) but it didn't change the result at all. I used `to_fen` as a hashing function, and it probably made it worse.
- Added IntMove which encodes a move in a single 64 `usize` word. This gives about 10% speedup and we have space for more flags.
