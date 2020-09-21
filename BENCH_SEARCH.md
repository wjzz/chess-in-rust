## make/copy results

Benchmarking position rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
Negamax d=1 | b1a3 |         40 | 2.10ms | 19 knps
Negamax d=2 | b1a3 |        440 | 45.06ms | 10 knps
Negamax d=3 | b1a3 |       9342 | 973.93ms | 10 knps
Negamax d=4 | b1a3 |     206623 | 23.27s | 9 knps

Benchmarking position rnbqk1nr/3p1ppp/1p1P4/p3p3/PbB5/2N2Q2/1PPBNPPP/2KRR3 b kq - 0 1
Negamax d=1 | b4c3 |         54 | 5.26ms | 10 knps
Negamax d=2 | f7f6 |       1261 | 177.34ms | 7 knps
Negamax d=3 | d8h4 |      33891 | 6.34s | 5 knps
Negamax d=4 | f7f6 |    1477885 | 217.73s | 7 knps

# Before removing pos[index]

Benchmarking position rnbqk1nr/3p1ppp/1pp1p3/p2P4/PbB1P3/2N2Q2/1PPBNPPP/2KRR3 b kq - 0 11
Negamax d = 3 | c6d5 |      42460 | 10.42s | 4 knps

Benchmarking position rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
Negamax d = 3 | b1a3 |       9342 | 1.38s | 7 knps

Benchmarking position rnbqk1nr/3p1ppp/1p1P4/p3p3/PbB5/2N2Q2/1PPBNPPP/2KRR3 b kq - 0 1
Negamax d = 3 | d8h4 |      33891 | 8.81s | 4 knps

# After removing the pos[index] indexing

Benchmarking position rnbqk1nr/3p1ppp/1pp1p3/p2P4/PbB1P3/2N2Q2/1PPBNPPP/2KRR3 b kq - 0 11
Negamax d = 3 | c6d5 |      42460 | 7.37s | 6 knps
Benchmarking position rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
Negamax d = 3 | b1a3 |       9342 | 919.60ms | 10 knps
Benchmarking position rnbqk1nr/3p1ppp/1p1P4/p3p3/PbB5/2N2Q2/1PPBNPPP/2KRR3 b kq - 0 1
Negamax d = 3 | d8h4 |      33891 | 6.13s | 6 knps
Benchmarking position rnbqk1nr/3p1ppp/1pp1p3/p2P4/PbB1P3/2N2Q2/1PPBNPPP/2KRR3 b kq - 0 11
Negamax d=1 | b4c3 |         66 | 5.99ms | 11 knps
Negamax d=2 | c6d5 |       1347 | 226.21ms | 6 knps
Negamax d=3 | c6d5 |      42460 | 7.54s | 6 knps
Negamax d=4 | c6d5 |    1681356 | 264.82s | 6 knps

## make/unmake results (before fixing indexes in en passant)
Benchmarking position rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
Negamax d=1 | b1a3 |         40 | 2.28ms | 18 knps
Negamax d=2 | b1a3 |        440 | 46.28ms | 10 knps
Negamax d=3 | b1a3 |       9342 | 974.71ms | 10 knps
Negamax d=4 | b1a3 |     206623 | 23.48s | 9 knps

Benchmarking position rnbqk1nr/3p1ppp/1p1P4/p3p3/PbB5/2N2Q2/1PPBNPPP/2KRR3 b kq - 0 1
Negamax d=1 | b4c3 |         54 | 5.08ms | 11 knps
Negamax d=2 | f7f6 |       1261 | 178.20ms | 7 knps
Negamax d=3 | d8h4 |      33891 | 6.36s | 5 knps
Negamax d=4 | f7f6 |    1477885 | 229.15s | 6 knps

Benchmarking position rnbqk1nr/3p1ppp/1pp1p3/p2P4/PbB1P3/2N2Q2/1PPBNPPP/2KRR3 b kq - 0 11
Negamax d=1 | b4c3 |         66 | 5.91ms | 11 knps
Negamax d=2 | c6d5 |       1347 | 216.15ms | 6 knps
Negamax d=3 | c6d5 |      42460 | 7.47s | 6 knps
Negamax d=4 | c6d5 |    1681356 | 272.75s | 6 knps

## make/unmake results (after fixing indexes in en passant)
Benchmarking position rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
Negamax d=1 | b1a3 |         40 | 2.61ms | 15 knps
Negamax d=2 | b1a3 |        440 | 28.02ms | 16 knps
Negamax d=3 | b1a3 |       9342 | 623.71ms | 15 knps
Negamax d=4 | b1a3 |     206623 | 14.72s | 14 knps

Benchmarking position rnbqk1nr/3p1ppp/1p1P4/p3p3/PbB5/2N2Q2/1PPBNPPP/2KRR3 b kq - 0 1
Negamax d=1 | b4c3 |         54 | 3.47ms | 16 knps
Negamax d=2 | f7f6 |       1261 | 128.90ms | 10 knps
Negamax d=3 | d8h4 |      33891 | 4.40s | 8 knps
Negamax d=4 | f7f6 |    1477885 | 169.03s | 9 knps

Benchmarking position rnbqk1nr/3p1ppp/1pp1p3/p2P4/PbB1P3/2N2Q2/1PPBNPPP/2KRR3 b kq - 0 11
Negamax d=1 | b4c3 |         66 | 4.05ms | 16 knps
Negamax d=2 | c6d5 |       1347 | 155.06ms | 9 knps
Negamax d=3 | c6d5 |      42460 | 5.40s | 8 knps
Negamax d=4 | c6d5 |    1681356 | 198.09s | 8 knps

## make/unmake alpha-beta results (after fixing indexes in en passant)

Benchmarking position rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
Alpha-beta d=1 | b1a3 |         21 | 1.29ms | 16 knps
Alpha-beta d=2 | b1a3 |         60 | 4.17ms | 14 knps
Alpha-beta d=3 | b1a3 |        586 | 37.36ms | 16 knps
Alpha-beta d=4 | b1a3 |       2316 | 166.75ms | 14 knps

Benchmarking position rnbqk1nr/3p1ppp/1p1P4/p3p3/PbB5/2N2Q2/1PPBNPPP/2KRR3 b kq - 0 1
Alpha-beta d=1 | b4c3 |         28 | 3.50ms | 8 knps
Alpha-beta d=2 | f7f6 |        872 | 95.20ms | 9 knps
Alpha-beta d=3 | d8h4 |       5737 | 800.75ms | 7 knps
Alpha-beta d=4 | f7f6 |     196745 | 21.41s | 9 knps

Benchmarking position rnbqk1nr/3p1ppp/1pp1p3/p2P4/PbB1P3/2N2Q2/1PPBNPPP/2KRR3 b kq - 0 11
Alpha-beta d=1 | b4c3 |         34 | 4.13ms | 8 knps
Alpha-beta d=2 | c6d5 |        336 | 40.47ms | 8 knps
Alpha-beta d=3 | c6d5 |       4147 | 554.19ms | 7 knps
Alpha-beta d=4 | c6d5 |      38610 | 4.63s | 8 knps

## make/unmake alpha-beta results with move ordering (after fixing indexes in en passant)

Benchmarking position rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
Alpha-beta move ord d=1 | b1a3 |         21 | 1.30ms | 16 knps
Alpha-beta move ord d=2 | h2h4 |         60 | 4.18ms | 14 knps
Alpha-beta move ord d=3 | b1a3 |        586 | 36.82ms | 16 knps
Alpha-beta move ord d=4 | h2h4 |       2369 | 188.91ms | 13 knps

Benchmarking position rnbqk1nr/3p1ppp/1p1P4/p3p3/PbB5/2N2Q2/1PPBNPPP/2KRR3 b kq - 0 1
Alpha-beta move ord d=1 | b4c3 |         28 | 3.67ms | 8 knps
Alpha-beta move ord d=2 | g8h6 |        871 | 90.57ms | 10 knps
Alpha-beta move ord d=3 | d8h4 |       5658 | 788.98ms | 7 knps
Alpha-beta move ord d=4 | g8f6 |     196075 | 21.63s | 9 knps

Benchmarking position rnbqk1nr/3p1ppp/1pp1p3/p2P4/PbB1P3/2N2Q2/1PPBNPPP/2KRR3 b kq - 0 11
Alpha-beta move ord d=1 | b4c3 |         34 | 4.09ms | 8 knps
Alpha-beta move ord d=2 | e6d5 |        164 | 20.96ms | 8 knps
Alpha-beta move ord d=3 | c6d5 |       8952 | 1.16s | 8 knps
Alpha-beta move ord d=4 | e6d5 |      16664 | 1.97s | 8 knps

## make/unmake PVS (after fixing indexes in en passant)

Benchmarking position rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
PVS d=1 | b1a3 |         21 | 1.30ms | 16 knps
PVS d=2 | b1a3 |         60 | 4.21ms | 14 knps
PVS d=3 | b1a3 |        550 | 35.23ms | 16 knps
PVS d=4 | b1a3 |       2304 | 159.17ms | 14 knps

Benchmarking position rnbqk1nr/3p1ppp/1p1P4/p3p3/PbB5/2N2Q2/1PPBNPPP/2KRR3 b kq - 0 1
PVS d=1 | b4c3 |         30 | 3.85ms | 8 knps
PVS d=2 | f7f6 |        921 | 95.37ms | 10 knps
PVS d=3 | d8h4 |       2633 | 363.51ms | 7 knps
PVS d=4 | f7f6 |     155902 | 16.55s | 9 knps

Benchmarking position rnbqk1nr/3p1ppp/1pp1p3/p2P4/PbB1P3/2N2Q2/1PPBNPPP/2KRR3 b kq - 0 11
PVS d=1 | b4c3 |         35 | 4.18ms | 8 knps
PVS d=2 | c6d5 |        368 | 44.47ms | 8 knps
PVS d=3 | c6d5 |       4685 | 607.87ms | 8 knps
PVS d=4 | c6d5 |      41459 | 4.83s | 9 knps

## Benchmarking all (after merging unmake into master)

Benchmarking position rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
Negamax d = 3        | b1a3 |       9342 | 668.10ms | 14 knps
Negamax d = 4        | b1a3 |     206623 |   15.14s | 14 knps
AlphaBeta d = 1      | b1a3 |         21 |   1.30ms | 16 knps
AlphaBeta d = 2      | h2h4 |         60 |   5.27ms | 11 knps
AlphaBeta d = 3      | b1a3 |        586 |  38.95ms | 15 knps
AlphaBeta d = 4      | h2h4 |       2369 | 177.50ms | 13 knps
PVS d = 4            | b1a3 |       2304 | 165.72ms | 14 knps

Benchmarking position rnbqk1nr/3p1ppp/1p1P4/p3p3/PbB5/2N2Q2/1PPBNPPP/2KRR3 b kq - 0 1
Negamax d = 3        | d8h4 |      33891 |    4.74s | 7 knps
AlphaBeta d = 1      | b4c3 |         28 |   4.21ms | 7 knps
AlphaBeta d = 2      | g8h6 |        871 |  94.52ms | 9 knps
AlphaBeta d = 3      | d8h4 |       5658 | 856.55ms | 7 knps
AlphaBeta d = 4      | g8f6 |     196075 |   23.27s | 8 knps
PVS d = 4            | f7f6 |     155902 |   17.32s | 9 knps

## Benchmarking all (after changing from Move to intmove)

Benchmarking position rnbqk1nr/3p1ppp/1pp1p3/p2P4/PbB1P3/2N2Q2/1PPBNPPP/2KRR3 b kq - 0 11
Negamax d = 3        | c6d5 |      42460 |    5.67s | 7 knps
AlphaBeta d = 1      | b4c3 |         34 |   4.81ms | 7 knps
AlphaBeta d = 2      | e6d5 |        164 |  24.06ms | 7 knps
AlphaBeta d = 3      | c6d5 |       8952 |    1.33s | 7 knps
AlphaBeta d = 4      | e6d5 |      16664 |    2.15s | 8 knps
PVS d = 4            | c6d5 |      41459 |    5.32s | 8 knps

Benchmarking position rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
Negamax d = 3        | b1a3 |       9342 | 677.72ms | 14 knps
AlphaBeta d = 1      | b1a3 |         21 |   1.31ms | 16 knps
AlphaBeta d = 2      | h2h4 |         60 |   4.23ms | 14 knps
AlphaBeta d = 3      | b1a3 |        586 |  38.37ms | 15 knps
AlphaBeta d = 4      | h2h4 |       2369 | 189.87ms | 12 knps
PVS d = 4            | b1a3 |       2304 | 161.12ms | 14 knps

Benchmarking position rnbqk1nr/3p1ppp/1p1P4/p3p3/PbB5/2N2Q2/1PPBNPPP/2KRR3 b kq - 0 1
Negamax d = 3        | d8h4 |      33891 |    4.25s | 8 knps
AlphaBeta d = 1      | b4c3 |         28 |   3.56ms | 8 knps
AlphaBeta d = 2      | g8h6 |        871 |  85.88ms | 10 knps
AlphaBeta d = 3      | d8h4 |       5658 | 750.82ms | 8 knps
AlphaBeta d = 4      | g8f6 |     196075 |   20.08s | 10 knps
PVS d = 4            | f7f6 |     155902 |   15.82s | 10 knps

Benchmarking position rnbqk1nr/3p1ppp/1pp1p3/p2P4/PbB1P3/2N2Q2/1PPBNPPP/2KRR3 b kq - 0 11
Negamax d = 3        | c6d5 |      42460 |    5.12s | 8 knps
AlphaBeta d = 1      | b4c3 |         34 |   4.13ms | 8 knps
AlphaBeta d = 2      | e6d5 |        164 |  32.19ms | 5 knps
AlphaBeta d = 3      | c6d5 |       8952 |    1.16s | 8 knps
AlphaBeta d = 4      | e6d5 |      16664 |    1.99s | 8 knps
PVS d = 4            | c6d5 |      41459 |    4.73s | 9 knps

# Benchmarks after improving the is_king_safe procedure

Benchmarking position rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
Negamax d = 3        | b1a3 |       9342 | 623.23ms | 15 knps
AlphaBeta d = 1      | b1a3 |         21 |   1.27ms | 17 knps
AlphaBeta d = 2      | h2h4 |         60 |   4.06ms | 15 knps
AlphaBeta d = 3      | b1a3 |        586 |  37.24ms | 16 knps
AlphaBeta d = 4      | h2h4 |       2369 | 170.81ms | 14 knps
PVS d = 4            | b1a3 |       2304 | 155.55ms | 15 knps

Benchmarking position rnbqk1nr/3p1ppp/1p1P4/p3p3/PbB5/2N2Q2/1PPBNPPP/2KRR3 b kq - 0 1
Negamax d = 3        | d8h4 |      33891 |    3.93s | 9 knps
AlphaBeta d = 1      | b4c3 |         28 |   3.13ms | 9 knps
AlphaBeta d = 2      | g8h6 |        871 |  80.63ms | 11 knps
AlphaBeta d = 3      | d8h4 |       5658 | 703.72ms | 8 knps
AlphaBeta d = 4      | g8f6 |     196075 |   18.85s | 10 knps
PVS d = 4            | f7f6 |     155902 |   15.11s | 10 knps

Benchmarking position rnbqk1nr/3p1ppp/1pp1p3/p2P4/PbB1P3/2N2Q2/1PPBNPPP/2KRR3 b kq - 0 11
Negamax d = 3        | c6d5 |      42460 |    4.89s | 9 knps
AlphaBeta d = 1      | b4c3 |         34 |   3.73ms | 9 knps
AlphaBeta d = 2      | e6d5 |        164 |  18.20ms | 9 knps
AlphaBeta d = 3      | c6d5 |       8952 |    1.09s | 8 knps
AlphaBeta d = 4      | e6d5 |      16664 |    1.85s | 9 knps
PVS d = 4            | c6d5 |      41459 |    4.58s | 9 knps

# After removing a lot of unnecessary vector copying in move generation

Benchmarking position rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
Negamax d = 3        | b1a3 |       9342 | 497.39ms | 19 knps
AlphaBeta d = 1      | b1a3 |         21 |   1.00ms | 21 knps
AlphaBeta d = 2      | h2h4 |         60 |   3.17ms | 19 knps
AlphaBeta d = 3      | b1a3 |        586 |  27.54ms | 21 knps
AlphaBeta d = 4      | h2h4 |       2369 | 127.23ms | 19 knps
PVS d = 4            | b1a3 |       2304 | 122.02ms | 19 knps

Benchmarking position rnbqk1nr/3p1ppp/1p1P4/p3p3/PbB5/2N2Q2/1PPBNPPP/2KRR3 b kq - 0 1
Negamax d = 3        | d8h4 |      33891 |    3.05s | 11 knps
AlphaBeta d = 1      | b4c3 |         28 |   2.48ms | 11 knps
AlphaBeta d = 2      | g8h6 |        871 |  56.73ms | 15 knps
AlphaBeta d = 3      | d8h4 |       5658 | 528.25ms | 11 knps
AlphaBeta d = 4      | g8f6 |     196075 |   12.97s | 15 knps
PVS d = 4            | f7f6 |     155902 |   10.10s | 15 knps

Benchmarking position rnbqk1nr/3p1ppp/1pp1p3/p2P4/PbB1P3/2N2Q2/1PPBNPPP/2KRR3 b kq - 0 11
Negamax d = 3        | c6d5 |      42460 |    3.80s | 11 knps
AlphaBeta d = 1      | b4c3 |         34 |   2.95ms | 12 knps
AlphaBeta d = 2      | e6d5 |        164 |  13.79ms | 12 knps
AlphaBeta d = 3      | c6d5 |       8952 | 854.65ms | 10 knps
AlphaBeta d = 4      | e6d5 |      16664 |    1.32s | 13 knps
PVS d = 4            | c6d5 |      41459 |    3.19s | 13 knps

# After moving to 0x88

Benchmarking position rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
Negamax d = 3        | b1a3 |       9342 | 407.04ms | 23 knps
AlphaBeta d = 1      | b1a3 |         21 | 827.61µs | 25 knps
AlphaBeta d = 2      | h2h4 |         60 |   2.58ms | 23 knps
AlphaBeta d = 3      | b1a3 |        586 |  23.49ms | 25 knps
AlphaBeta d = 4      | h2h4 |       2369 | 104.58ms | 23 knps
PVS d = 4            | b1a3 |       2302 | 103.27ms | 22 knps

Benchmarking position rnbqk1nr/3p1ppp/1p1P4/p3p3/PbB5/2N2Q2/1PPBNPPP/2KRR3 b kq - 0 1
Negamax d = 3        | d8h4 |      33891 |    2.57s | 13 knps
AlphaBeta d = 1      | b4c3 |         28 |   2.10ms | 13 knps
AlphaBeta d = 2      | g8f6 |        842 |  47.74ms | 18 knps
AlphaBeta d = 3      | d8h4 |       5357 | 420.75ms | 13 knps
AlphaBeta d = 4      | g8f6 |     201297 |   11.30s | 18 knps
PVS d = 4            | f7f6 |     170579 |    9.67s | 18 knps

Benchmarking position rnbqk1nr/3p1ppp/1pp1p3/p2P4/PbB1P3/2N2Q2/1PPBNPPP/2KRR3 b kq - 0 11
Negamax d = 3        | c6d5 |      42460 |    3.21s | 13 knps
AlphaBeta d = 1      | b4c3 |         34 |   2.54ms | 13 knps
AlphaBeta d = 2      | e6d5 |        163 |  12.34ms | 13 knps
AlphaBeta d = 3      | c6d5 |       8123 | 647.24ms | 13 knps
AlphaBeta d = 4      | e6d5 |      16360 |    1.10s | 15 knps
PVS d = 4            | c6d5 |      40431 |    2.63s | 15 knps

**After caching king's location (shortener) NOTE: AlphaBeta runs iterative deepening for move ordering**

Benchmarking position rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
Negamax d = 3        | b1a3 |       9342 | 372.26ms | 25 knps
AlphaBeta d = 4      | h2h4 |       3036 | 123.44ms | 25 knps
PVS d = 4            | b1a3 |       2302 |  93.20ms | 25 knps

Benchmarking position rnbqk1nr/3p1ppp/1p1P4/p3p3/PbB5/2N2Q2/1PPBNPPP/2KRR3 b kq - 0 1
Negamax d = 3        | d8h4 |      33891 |    2.38s | 14 knps
AlphaBeta d = 4      | g8f6 |     207524 |   10.54s | 20 knps
PVS d = 4            | f7f6 |     170579 |    8.48s | 20 knps

Benchmarking position rnbqk1nr/3p1ppp/1pp1p3/p2P4/PbB1P3/2N2Q2/1PPBNPPP/2KRR3 b kq - 0 11
Negamax d = 3        | c6d5 |      42460 |    3.06s | 14 knps
AlphaBeta d = 4      | e6d5 |      24680 |    1.66s | 15 knps
PVS d = 4            | c6d5 |      40431 |    2.41s | 17 knps

**Add a smarter is-king-in-check checker**

Benchmarking position rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
Negamax d = 3        | b1a3 |       9342 | 142.50ms | 66 knps
Negamax d = 4        | b1a3 |     206623 |    3.33s | 62 knps
AlphaBeta d = 4      | h2h4 |       3036 |  50.14ms | 61 knps
AlphaBeta d = 5      | b2b3 |      44399 | 704.70ms | 63 knps
PVS d = 4            | b1a3 |       2302 |  37.47ms | 61 knps
PVS d = 5            | b2b3 |      41172 | 649.12ms | 63 knps

Benchmarking position rnbqk1nr/3p1ppp/1p1P4/p3p3/PbB5/2N2Q2/1PPBNPPP/2KRR3 b kq - 0 1
Negamax d = 3        | d8h4 |      33890 | 897.31ms | 38 knps
Negamax d = 4        | f7f6 |    1478005 |   29.57s | 50 knps
AlphaBeta d = 4      | g8f6 |     207589 |    4.12s | 50 knps
AlphaBeta d = 5      | d8f6 |    1053922 |   28.35s | 37 knps
PVS d = 4            | f7f6 |     170614 |    3.27s | 52 knps
PVS d = 5            | d8f6 |     454963 |   12.96s | 35 knps

Benchmarking position rnbqk1nr/3p1ppp/1pp1p3/p2P4/PbB1P3/2N2Q2/1PPBNPPP/2KRR3 b kq - 0 11
Negamax d = 3        | c6d5 |      42460 |    1.04s | 41 knps
Negamax d = 4        | c6d5 |    1681549 |   37.38s | 45 knps
AlphaBeta d = 4      | e6d5 |      24686 | 595.59ms | 41 knps
AlphaBeta d = 5      | c6d5 |     975253 |   26.83s | 36 knps
PVS d = 4            | c6d5 |      40451 | 913.13ms | 44 knps
PVS d = 5            | c6d5 |     500694 |   13.75s | 36 knps
