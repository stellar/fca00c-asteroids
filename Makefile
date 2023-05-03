default: test

build:
	cargo build --target wasm32-unknown-unknown --release
	cd target/wasm32-unknown-unknown/release/ && \
		for i in *.wasm ; do \
			ls -l "$$i"; \
		done

build-optimized:
	cargo +nightly build --target wasm32-unknown-unknown --release -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort
	cd target/wasm32-unknown-unknown/release/ && \
		for i in *.wasm ; do \
			soroban contract optimize --wasm "$$i" --wasm-out "$$i.tmp" && mv "$$i.tmp" "$$i"; \
			ls -l "$$i"; \
		done

test:
	cargo test fca00c_fast -- --nocapture

test-budget:
	cargo test fca00c_budget -- --nocapture

clean:
	cargo clean

fmt:
	cargo fmt --all