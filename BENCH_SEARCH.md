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
Negamax d = 1 | b4c3 |         66 | 7.77ms | 8 knps
Negamax d = 2 | c6d5 |       1347 | 296.53ms | 5 knps
Negamax d = 3 | c6d5 |      42460 | 10.42s | 4 knps

Benchmarking position rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
Negamax d = 1 | b1a3 |         40 | 2.71ms | 15 knps
Negamax d = 2 | b1a3 |        440 | 61.95ms | 7 knps
Negamax d = 3 | b1a3 |       9342 | 1.38s | 7 knps

Benchmarking position rnbqk1nr/3p1ppp/1p1P4/p3p3/PbB5/2N2Q2/1PPBNPPP/2KRR3 b kq - 0 1
Negamax d = 1 | b4c3 |         54 | 7.03ms | 8 knps
Negamax d = 2 | f7f6 |       1261 | 247.46ms | 5 knps
Negamax d = 3 | d8h4 |      33891 | 8.81s | 4 knps

# After removing the pos[index] indexing

Benchmarking position rnbqk1nr/3p1ppp/1pp1p3/p2P4/PbB1P3/2N2Q2/1PPBNPPP/2KRR3 b kq - 0 11
Negamax d = 1 | b4c3 |         66 | 5.35ms | 12 knps
Negamax d = 2 | c6d5 |       1347 | 210.26ms | 6 knps
Negamax d = 3 | c6d5 |      42460 | 7.37s | 6 knps
Benchmarking position rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
Negamax d = 1 | b1a3 |         40 | 1.75ms | 23 knps
Negamax d = 2 | b1a3 |        440 | 37.80ms | 12 knps
Negamax d = 3 | b1a3 |       9342 | 919.60ms | 10 knps
Benchmarking position rnbqk1nr/3p1ppp/1p1P4/p3p3/PbB5/2N2Q2/1PPBNPPP/2KRR3 b kq - 0 1
Negamax d = 1 | b4c3 |         54 | 4.84ms | 11 knps
Negamax d = 2 | f7f6 |       1261 | 185.74ms | 7 knps
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
Negamax d = 1        | b1a3 |         40 |   1.27ms | 31 knps
Negamax d = 2        | b1a3 |        440 |  26.76ms | 16 knps
Negamax d = 3        | b1a3 |       9342 | 668.10ms | 14 knps
Negamax d = 4        | b1a3 |     206623 |   15.14s | 14 knps
AlphaBeta d = 1      | b1a3 |         21 |   1.30ms | 16 knps
AlphaBeta d = 2      | h2h4 |         60 |   5.27ms | 11 knps
AlphaBeta d = 3      | b1a3 |        586 |  38.95ms | 15 knps
AlphaBeta d = 4      | h2h4 |       2369 | 177.50ms | 13 knps
PVS d = 1            | b1a3 |         21 |   1.35ms | 16 knps
PVS d = 2            | b1a3 |         60 |   4.23ms | 14 knps
PVS d = 3            | b1a3 |        550 |  40.15ms | 14 knps
PVS d = 4            | b1a3 |       2304 | 165.72ms | 14 knps

Benchmarking position rnbqk1nr/3p1ppp/1p1P4/p3p3/PbB5/2N2Q2/1PPBNPPP/2KRR3 b kq - 0 1
Negamax d = 1        | b4c3 |         54 |   3.50ms | 15 knps
Negamax d = 2        | f7f6 |       1261 | 136.02ms | 9 knps
Negamax d = 3        | d8h4 |      33891 |    4.74s | 7 knps
AlphaBeta d = 1      | b4c3 |         28 |   4.21ms | 7 knps
AlphaBeta d = 2      | g8h6 |        871 |  94.52ms | 9 knps
AlphaBeta d = 3      | d8h4 |       5658 | 856.55ms | 7 knps
AlphaBeta d = 4      | g8f6 |     196075 |   23.27s | 8 knps
PVS d = 1            | b4c3 |         30 |   3.92ms | 8 knps
PVS d = 2            | f7f6 |        921 | 126.50ms | 7 knps
PVS d = 3            | d8h4 |       2633 | 396.21ms | 7 knps
PVS d = 4            | f7f6 |     155902 |   17.32s | 9 knps

Benchmarking position rnbqk1nr/3p1ppp/1pp1p3/p2P4/PbB1P3/2N2Q2/1PPBNPPP/2KRR3 b kq - 0 11
Negamax d = 1        | b4c3 |         66 |   4.30ms | 15 knps
Negamax d = 2        | c6d5 |       1347 | 155.59ms | 9 knps
Negamax d = 3        | c6d5 |      42460 |    5.67s | 7 knps
AlphaBeta d = 1      | b4c3 |         34 |   4.81ms | 7 knps
AlphaBeta d = 2      | e6d5 |        164 |  24.06ms | 7 knps
AlphaBeta d = 3      | c6d5 |       8952 |    1.33s | 7 knps
AlphaBeta d = 4      | e6d5 |      16664 |    2.15s | 8 knps
PVS d = 1            | b4c3 |         35 |   4.57ms | 8 knps
PVS d = 2            | c6d5 |        368 |  46.39ms | 8 knps
PVS d = 3            | c6d5 |       4685 | 677.69ms | 7 knps
PVS d = 4            | c6d5 |      41459 |    5.32s | 8 knps
