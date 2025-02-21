# This justfile is imported in subprojects and it is used to compile individual
# examples.

# user settings should be placed into .env file in some ancestor directory
set dotenv-load

# llvm flags
export RUSTFLAGS := "-C llvm-args=--sbf-expand-memcpy-in-order -C llvm-args=--combiner-store-merging=false -C llvm-args=--combiner-load-merging=false -C llvm-args=--aggressive-instcombine-max-scan-instrs=0 -C llvm-args=--combiner-reduce-load-op-store-width=false -C llvm-args=--combiner-shrink-load-replace-store-with-store=false -C strip=none -C debuginfo=2"

# features used when compiling target Rust code
export CARGO_FEATURES := env_var_or_default("CARGO_FEATURES", "")

build-sbf extra_features="":
	echo "env RUSTFLAGS=$RUSTFLAGS"
	echo "env CARGO_FEATURES=$CARGO_FEATURES"
	cargo +solana build-sbf --features certora {{ extra_features }} ${CARGO_FEATURES}
