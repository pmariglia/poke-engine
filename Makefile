test:
	cargo test

test_all:
	cargo test --no-default-features --features "gen9"
	cargo test --no-default-features --features "gen8"
	cargo test --no-default-features --features "gen7"
	cargo test --no-default-features --features "gen6"
	cargo test --no-default-features --features "gen5"
	cargo test --no-default-features --features "gen4"