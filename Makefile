default: test

build:
	cargo build --target wasm32-unknown-unknown --release
	cd target\wasm32-unknown-unknown\release && \
	for %%i in (*.wasm) do ( dir /q "%%i" )

build-optimized:
	cargo +nightly build --target wasm32-unknown-unknown --release -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort
	cd target/wasm32-unknown-unknown/release/ && \
	for %%i in (*.wasm) do ( wasm-opt -Oz -c -mvp "%%i" -o "%%i.tmp" && move "%%i.tmp" "%%i" && dir /q "%%i" )

test:
	cargo test fca00c_fast -- --nocapture

test-budget:
	cargo test fca00c_budget -- --nocapture

clean:
	cargo clean

fmt:
	cargo fmt --all