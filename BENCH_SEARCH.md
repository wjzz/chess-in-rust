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
