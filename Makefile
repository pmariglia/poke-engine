.PHONY: release

dev:
	virtualenv -p python3 venv
	. venv/bin/activate && pip install -r poke-engine-py/requirements.txt && pip install -r poke-engine-py/requirements-dev.txt && cd poke-engine-py && maturin develop --features="poke-engine/gen4"

upload_python_bindings:
	cd poke-engine-py && ./build_and_publish

upload_rust_lib:
	cargo publish --features "gen4"

release:
	./release

fmt:
	cargo fmt
	ruff format poke-engine-py

gen1:
	cargo build --release --features gen1 --no-default-features

gen2:
	cargo build --release --features gen2 --no-default-features

gen3:
	cargo build --release --features gen3 --no-default-features

gen4:
	cargo build --release --features gen4 --no-default-features

gen5:
	cargo build --release --features gen5 --no-default-features

gen6:
	cargo build --release --features gen6 --no-default-features

gen7:
	cargo build --release --features gen7 --no-default-features

gen8:
	cargo build --release --features gen8 --no-default-features

gen9:
	cargo build --release --features gen9,terastallization --no-default-features

pytest:
	. venv/bin/activate && pytest --rootdir=poke-engine-py/python poke-engine-py/python/tests

test: pytest
	cargo test --no-default-features --features "terastallization"
	cargo test --no-default-features --features "gen9"
	cargo test --no-default-features --features "gen8"
	cargo test --no-default-features --features "gen7"
	cargo test --no-default-features --features "gen6"
	cargo test --no-default-features --features "gen5"
	cargo test --no-default-features --features "gen4"
	cargo test --no-default-features --features "gen3"
	cargo test --no-default-features --features "gen2"
	cargo test --no-default-features --features "gen1"

install_ci:
	pip install -r poke-engine-py/requirements.txt
	pip install -r poke-engine-py/requirements-dev.txt
	cd poke-engine-py && maturin develop --features="poke-engine/gen4"

fmt_ci:
	cargo fmt -- --check
	ruff format --check poke-engine-py

test_ci:
	pytest --rootdir=poke-engine-py/python poke-engine-py/python/tests
	cargo test --no-default-features --features "gen9"
	cargo test --no-default-features --features "gen8"
	cargo test --no-default-features --features "gen7"
	cargo test --no-default-features --features "gen6"
	cargo test --no-default-features --features "gen5"
	cargo test --no-default-features --features "gen4"
	cargo test --no-default-features --features "gen3"
	cargo test --no-default-features --features "gen2"
	cargo test --no-default-features --features "gen1"

ci: install_ci fmt_ci test_ci
