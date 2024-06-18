gen4:
	./build gen4

gen5:
	./build gen5

gen6:
	./build gen6

gen7:
	./build gen7

gen8:
	./build gen8 &&

test:
	cargo test --no-default-features --features "gen9"
	cargo test --no-default-features --features "gen8"
	cargo test --no-default-features --features "gen7"
	cargo test --no-default-features --features "gen6"
	cargo test --no-default-features --features "gen5"
	cargo test --no-default-features --features "gen4"
	cargo test --test test_last_used_move --no-default-features --features "gen9,last_used_move"
	cargo test --test test_damage_dealt --no-default-features --features "gen9,damage_dealt"