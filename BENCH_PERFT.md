# Perft

## Naive implementation with clone

### Depth 4 (inclusive)

`cargo run`

perf imm 4 = 197281 | correct: true
real    2m2,061s

`cargo run --release`

perf imm 4 = 197281 | correct: true
real    0m6,434s

**After en passant rule is added**
perf imm 4 = 197281 | correct: true
real	0m8,446s

**Parallel ver: After en passant rule is added**
perf imm 4 = 197281 | correct: true
real	0m1,687s

### Depth 5  (inclusive)

perf imm 5 = 4865351 | correct: false
real    2m37,604s

**After en passant rule is added**
perf imm 5 = 4865609 | correct: true
real	3m32,410s

**Parallel ver: After en passant rule is added**
perf imm 5 = 4865609 | correct: true
real	0m55,846s

**New parallel ver: After castling and en passant are added**

4 threads | perf imm 5 =    4865609 | correct: true | total time   49.77s
perf imm 5 =    4865609 | correct: true | total time   58.34s

### Depth 6  (inclusive)

**After en passant rule is added**

perf imm 6 = 119060324 | correct: true
real	61m58,892s

**Parallel ver: After en passant rule is added**
perf imm 6 = 119060324 | correct: true
real	20m55,193s

### Depth 7 (inclusive)

**Parallel ver: After en passant rule is added**
perf imm 6 = 119060324 | correct: true
^C (DIDN'T FINISH)
real	376m13,488s (= 6h)

### Benchmark

**make sure to turn on debug symbols in .toml**
valgrind --tool=callgrind --dump-instr=yes --collect-jumps=yes --simulate-cache=yes target/release/rust-chess
kcachegrind callgrind.out.19316

**Before field indexing by coord is removed**

Using 4 threads.
perf imm 5 =    4865609 | correct: true | total time   42.40s

**After going for normal indexing**

Using 4 threads.
perf imm 5 =    4865609 | correct: true | total time    3.33s
perf imm 6 =  119060324 | correct: true | total time   86.10s   [  1.2 min]
perf imm 7 = 3195901860 | correct: true | total time 3025.23s   [ 50.0 min]

**Using unmake move with fixed unmake move**

Using 4 threads.
perf imm 5 =    4865609 | correct: true | total time    2.08s
perf imm 6 =  119060324 | correct: true | total time   63.12s

**After changing move repr**

Using 4 threads.
perf imm 5 =    4865609 | correct: true | total time    2.02s
perf imm 6 =  119060324 | correct: true | total time   58.28s
perf imm 7 = 3195901860 | correct: true | total time 1721.53s  [ 29.0 min]

**After removing unnecessary vector copying in move gen**

Using 4 threads.
perf imm 5 =    4865609 | correct: true | total time    1.62s
perf imm 6 =  119060324 | correct: true | total time   44.27s
perf imm 7 = 3195901860 | correct: true | total time 1472.22s [ 24.5 min]

**After removing coord2index conversions and changing en passant repr to index**

Using 4 threads.
perf imm 5 =    4865609 | correct: true | total time    1.53s
perf imm 6 =  119060324 | correct: true | total time   42.26s

**After initial switching to 0x88 board representation**

Using 4 threads.
perf imm 5 =    4865609 | correct: true | total time    1.32s
perf imm 6 =  119060324 | correct: true | total time   38.53s

**Same as above, nicer stats**

Using 4 threads.
perf imm 5 =    4865609 | correct: true | total time    1.40s |  3483 knps |   871 knps (thread)
perf imm 6 =  119060324 | correct: true | total time   44.32s |  2686 knps |   672 knps (thread)

**Add king location caching**

Using 4 threads.
perf imm 5 =    4865609  | correct: true  | total time     1.33s |  3652 knps |   913 knps (thread)
perf imm 6 =  119060324  | correct: true  | total time    35.13s |  3390 knps |   847 knps (thread)
perf imm 7 = 3195901860  | correct: true  | total time  1148.39s |  2783 knps |   696 knps (thread)
perf imm 8 = 84998979184 | correct: false | total time 25187.36s |  3375 knps |   844 knps (thread)
wrong answer after 7 hours
got       84_998_979_184
should be 84_998_978_956

**Add a smarter is-king-in-check checker**

Using 4 threads.
perf imm 5 =    4865609 | correct: true | total time 469.53ms | 10363 knps |  2591 knps (thread)
perf imm 6 =  119060324 | correct: true | total time   12.20s |  9763 knps |  2441 knps (thread)
perf imm 7 = 3195901860 | correct: true | total time  460.39s |  6942 knps |  1735 knps (thread) [<7 mins!!]
