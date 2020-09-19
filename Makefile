.PHONY: bot bench perft

bot:
	@cargo build --release --bin bot
	@cp target/release/bot ../lichess-bot/engines
	@echo "Bot released"

bench:
	@cargo run --bin benchmark --release

perft:
	@cargo run --bin rust-chess --release
