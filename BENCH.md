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
perf imm 1 =         20 | correct: true | total time 380.72µs
perf imm 2 =        400 | correct: true | total time 576.64µs
perf imm 3 =       8902 | correct: true | total time  13.75ms
perf imm 4 =     197281 | correct: true | total time 140.97ms
perf imm 5 =    4865609 | correct: true | total time    3.30s
