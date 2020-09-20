.PHONY: bot bench perft valgrind

bot:
	@cargo build --release --bin bot
	@cp target/release/bot ../lichess-bot/engines
	@echo "Bot released"

bench:
	@cargo run --bin benchmark --release

perft:
	@cargo run --bin rust-chess --release

valgrind:
	@cargo build --release --bin benchmark
	valgrind --tool=callgrind --dump-instr=yes --collect-jumps=yes --simulate-cache=yes target/release/benchmark
	kcachegrind callgrind.out*

test:
	cargo test
	cargo test -- --ignored
