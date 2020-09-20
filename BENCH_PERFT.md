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
perf imm 1 =         20 | correct: true | total time 833.79µs
perf imm 2 =        400 | correct: true | total time   3.77ms
perf imm 3 =       8902 | correct: true | total time  75.82ms
perf imm 4 =     197281 | correct: true | total time    1.62s
perf imm 5 =    4865609 | correct: true | total time   42.40s

**After going for normal indexing**

Using 4 threads.
perf imm 1 =         20 | correct: true | total time 535.44µs
perf imm 2 =        400 | correct: true | total time 615.69µs
perf imm 3 =       8902 | correct: true | total time  12.13ms
perf imm 4 =     197281 | correct: true | total time 215.28ms
perf imm 5 =    4865609 | correct: true | total time    3.33s
perf imm 6 =  119060324 | correct: true | total time   86.10s   [  1.2 min]
perf imm 7 = 3195901860 | correct: true | total time 3025.23s   [ 50.0 min]

**Using unmake move with fixed unmake move**

Using 4 threads.
perf imm 1 =         20 | correct: true | total time 219.89µs
perf imm 2 =        400 | correct: true | total time 514.40µs
perf imm 3 =       8902 | correct: true | total time   7.13ms
perf imm 4 =     197281 | correct: true | total time  88.68ms
perf imm 5 =    4865609 | correct: true | total time    2.08s
perf imm 6 =  119060324 | correct: true | total time   63.12s

**After changing move repr**

Using 4 threads.
perf imm 1 =         20 | correct: true | total time 483.61µs
perf imm 2 =        400 | correct: true | total time 341.03µs
perf imm 3 =       8902 | correct: true | total time   8.59ms
perf imm 4 =     197281 | correct: true | total time  96.78ms
perf imm 5 =    4865609 | correct: true | total time    2.02s
perf imm 6 =  119060324 | correct: true | total time   58.28s
perf imm 7 = 3195901860 | correct: true | total time 1721.53s  [ 29.0 min]

**After removing unnecessary vector copying in move gen**

Using 4 threads.
perf imm 1 =         20 | correct: true | total time 243.58µs
perf imm 2 =        400 | correct: true | total time 372.28µs
perf imm 3 =       8902 | correct: true | total time   5.39ms
perf imm 4 =     197281 | correct: true | total time  92.78ms
perf imm 5 =    4865609 | correct: true | total time    1.62s
perf imm 6 =  119060324 | correct: true | total time   44.27s
perf imm 7 = 3195901860 | correct: true | total time 1472.22s [ 24.5 min]

**After removing coord2index conversions and changing en passant repr to index**

Using 4 threads.
perf imm 1 =         20 | correct: true | total time 232.80µs
perf imm 2 =        400 | correct: true | total time 356.15µs
perf imm 3 =       8902 | correct: true | total time   7.60ms
perf imm 4 =     197281 | correct: true | total time  75.68ms
perf imm 5 =    4865609 | correct: true | total time    1.53s
perf imm 6 =  119060324 | correct: true | total time   42.26s

**After initial switching to 0x88 board representation**

Using 4 threads.
perf imm 1 =         20 | correct: true | total time 828.28µs
perf imm 2 =        400 | correct: true | total time 351.85µs
perf imm 3 =       8902 | correct: true | total time   6.84ms
perf imm 4 =     197281 | correct: true | total time  74.34ms
perf imm 5 =    4865609 | correct: true | total time    1.32s
perf imm 6 =  119060324 | correct: true | total time   38.53s
