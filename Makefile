lint:
	cargo clippy --all-targets --all-features -- -D warnings

test:
	cargo test --all-targets --all-features