default: test

build:
	cargo build --target wasm32v1-none --release
	cd target/wasm32v1-none/release/ && \
		for i in *.wasm ; do \
			ls -l "$$i"; \
		done

build-optimized: build
	cd target/wasm32v1-none/release/ && \
		for i in *.wasm ; do \
			stellar contract optimize --wasm "$$i" --wasm-out "$$i.tmp" && mv "$$i.tmp" "$$i"; \
			ls -l "$$i"; \
		done

# Rebuild the prebuilt game engine from contracts/_game_engine (maintainers only).
build-engine:
	cargo build -p soroban-asteroids-game-engine --target wasm32v1-none --release
	cp target/wasm32v1-none/release/soroban_asteroids_game_engine.wasm contracts/game_engine.wasm
	cargo test -p soroban-asteroids-game-engine

test:
	cargo test fca00c_fast -- --nocapture

test-budget:
	cargo test fca00c_budget -- --nocapture

clean:
	cargo clean

fmt:
	cargo fmt --all
