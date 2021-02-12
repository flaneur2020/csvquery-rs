build:
	cargo build

lint:
	cargo fmt
	cargo clippy -- -D warnings

clean:
	cargo clean

.PHONY: build lint clean
