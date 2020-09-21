.PHONY: bot bench perft valgrind

bot:
	@cargo build --release --bin bot
	@cp target/release/bot ../lichess-bot/engines
	@echo "Bot released"

bench:
	@cargo run --bin benchmark --release

main:
	@cargo run --bin rust-chess --release

perft:
	@cargo run --bin perft --release

split:
	@cargo run --bin split --release

valgrind:
	@cargo build --release --bin benchmark
	@cargo build --release --bin rust-chess
	valgrind --tool=callgrind --dump-instr=yes --collect-jumps=yes --simulate-cache=yes target/release/rust-chess
	# valgrind --tool=callgrind --dump-instr=yes --collect-jumps=yes --simulate-cache=yes target/release/benchmark
	kcachegrind callgrind.out*

test:
	cargo test
	cargo test -- --ignored
