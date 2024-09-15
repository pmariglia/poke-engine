dev:
	virtualenv -p python3 venv
	. venv/bin/activate && pip install -r poke-engine-py/requirements.txt && pip install -r poke-engine-py/requirements-dev.txt && cd poke-engine-py && maturin develop --features="poke-engine/gen4"

upload_python_bindings:
	cd poke-engine-py && ./build_and_publish.sh

upload_rust_lib:
	cargo publish --features "gen4"

new-tag:
	@current_tag=$$(git describe --tags `git rev-list --tags --max-count=1`); \
	echo "Current tag: $$current_tag"; \
	read -p "Enter the new tag: " new_tag; \
	git tag -a "$$new_tag" -m "$$new_tag"; \
	git-cliff -l -p CHANGELOG.md; \
	git add CHANGELOG.md; \
	git commit -m "Update CHANGELOG.md for $$new_tag"; \
	git push origin main --tags

fmt:
	cargo fmt
	ruff format poke-engine-py

gen4:
	./build gen4

gen5:
	./build gen5

gen6:
	./build gen6

gen7:
	./build gen7

gen8:
	./build gen8

pytest:
	. venv/bin/activate && pytest --rootdir=poke-engine-py/python poke-engine-py/python/tests

test: pytest
	cargo test --no-default-features --features "gen9"
	cargo test --no-default-features --features "gen8"
	cargo test --no-default-features --features "gen7"
	cargo test --no-default-features --features "gen6"
	cargo test --no-default-features --features "gen5"
	cargo test --no-default-features --features "gen4"
	cargo test --test test_last_used_move --no-default-features --features "gen9,last_used_move"
	cargo test --test test_damage_dealt --no-default-features --features "gen9,damage_dealt"

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
	cargo test --test test_last_used_move --no-default-features --features "gen9,last_used_move"
	cargo test --test test_damage_dealt --no-default-features --features "gen9,damage_dealt"

ci: install_ci fmt_ci test_ci
