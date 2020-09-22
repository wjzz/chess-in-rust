# Using inefficent eval fun

#1 Comparing position 2rk2r1/3p2rr/8/1Q3Q2/6B1/8/3Q4/K7 w - - 0 1
Negamax         d=1 | f5h7 | 14.5 |         70 |   1.27ms | 55 knps
PVS             d=1 | f5h7 | 14.5 |         72 |   1.26ms | 57 knps
AlphaBeta       d=1 | f5h7 | 14.5 |         70 |   1.28ms | 55 knps
AlphaBeta ItD   d=1 | f5h7 | 14.5 |         70 |   1.24ms | 56 knps

Negamax         d=2 | d2a5 | 9.5 |       1721 |  53.50ms | 32 knps
PVS             d=2 | d2a5 | 9.5 |        212 |   6.21ms | 34 knps
AlphaBeta       d=2 | d2a5 | 9.5 |        207 |   6.04ms | 34 knps
AlphaBeta ItD   d=2 | d2a5 | 9.5 |        277 |   7.46ms | 37 knps

Negamax         d=3 | d2a5 | 14.5 |     103778 |    1.99s | 52 knps
PVS             d=3 | d2a5 | 14.5 |       9587 | 186.17ms | 51 knps
AlphaBeta       d=3 | d2a5 | 14.5 |       9251 | 180.31ms | 51 knps
AlphaBeta ItD   d=3 | d2a5 | 14.5 |       9528 | 188.28ms | 51 knps

PVS             d=4 | a1b2 | 9.5 |      27915 | 853.93ms | 33 knps
AlphaBeta       d=4 | a1b2 | 9.5 |      32341 | 987.59ms | 33 knps
AlphaBeta ItD   d=4 | a1b2 | 9.5 |      41869 |    1.22s | 34 knps

PVS             d=5 | d2g5 | 10000.0 |     617500 |   12.50s | 49 knps
AlphaBeta       d=5 | d2g5 | 10000.0 |     275827 |    5.78s | 48 knps
AlphaBeta ItD   d=5 | d2g5 | 10000.0 |     317696 |    6.90s | 46 knps

# After using tables

Negamax         d=1 | f5h7 | 14.5 |         70 |   1.23ms | 57 knps
PVS             d=1 | f5h7 | 14.5 |         72 |   1.25ms | 58 knps
AlphaBeta       d=1 | f5h7 | 14.5 |         70 |   1.23ms | 57 knps
AlphaBeta ItD   d=1 | f5h7 | 14.5 |         70 |   1.21ms | 58 knps

Negamax         d=2 | d2a5 | 9.5 |       1721 |  54.62ms | 32 knps
PVS             d=2 | d2a5 | 9.5 |        212 |   6.24ms | 34 knps
AlphaBeta       d=2 | d2a5 | 9.5 |        207 |   6.14ms | 34 knps
AlphaBeta ItD   d=2 | d2a5 | 9.5 |        277 |   7.38ms | 38 knps

Negamax         d=3 | d2a5 | 14.5 |     103778 |    2.02s | 51 knps
PVS             d=3 | d2a5 | 14.5 |       9587 | 189.44ms | 51 knps
AlphaBeta       d=3 | d2a5 | 14.5 |       9251 | 182.47ms | 51 knps
AlphaBeta ItD   d=3 | d2a5 | 14.5 |       9528 | 188.71ms | 50 knps

PVS             d=4 | a1b2 | 9.5 |      27915 | 844.79ms | 33 knps
AlphaBeta       d=4 | a1b2 | 9.5 |      32341 | 976.21ms | 33 knps
AlphaBeta ItD   d=4 | a1b2 | 9.5 |      41869 |    1.16s | 36 knps

PVS             d=5 | d2g5 | 10000.0 |     617500 |   12.36s | 50 knps
AlphaBeta       d=5 | d2g5 | 10000.0 |     275827 |    5.61s | 49 knps
AlphaBeta ItD   d=5 | d2g5 | 10000.0 |     317696 |    6.87s | 46 knps

