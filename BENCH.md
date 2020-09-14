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
 Thread 1 finished mv  1/20	A2->A3 after 5.97s
 Thread 3 finished mv  3/20	B1->A3 after 6.53s
 Thread 2 finished mv  2/20	A2->A4 after 7.14s
 Thread 4 finished mv  4/20	B1->C3 after 7.75s
 Thread 1 finished mv  5/20	B2->B3 after 7.09s
 Thread 3 finished mv  7/20	C2->C3 after 7.34s
 Thread 2 finished mv  6/20	B2->B4 after 7.02s
 Thread 4 finished mv  8/20	C2->C4 after 7.87s
 Thread 1 finished mv  9/20	D2->D3 after 10.70s
 Thread 2 finished mv 10/20	D2->D4 after 11.82s
 Thread 3 finished mv 11/20	E2->E3 after 12.98s
 Thread 4 finished mv 12/20	E2->E4 after 13.14s
 Thread 1 finished mv 13/20	F2->F3 after 5.98s
 Thread 2 finished mv 14/20	F2->F4 after 6.61s
 Thread 3 finished mv 15/20	G1->F3 after 7.67s
 Thread 4 finished mv 16/20	G1->H3 after 6.60s
 Thread 1 finished mv 17/20	G2->G3 after 7.09s
 Thread 2 finished mv 18/20	G2->G4 after 7.03s
 Thread 3 finished mv 19/20	H2->H3 after 5.97s
 Thread 4 finished mv 20/20	H2->H4 after 7.04s
perf imm 5 =    4865609 | correct: true | total time   42.40s
