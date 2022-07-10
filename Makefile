
all: format build

clean:
	@cargo clean

format:
	@cargo fmt

run:
	@cargo run --package wildkv-server --bin wildkv-server -- --config ../../config.toml
