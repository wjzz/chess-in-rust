# Perft

## Naive implementation with clone

### Depth 4 (inclusive)

`cargo run`

perf imm 4 = 197281 | correct: true

real    2m2,061s

`cargo run --release`

perf imm 4 = 197281 | correct: true

real    0m6,434s

### Depth 5  (inclusive)

perf imm 5 = 4865351 | correct: false

real    2m37,604s