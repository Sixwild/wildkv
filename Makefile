
all: format build

clean:
	@cargo clean

format:
	@cargo fmt

run:
	@cargo run -q --package wildkv-server --bin wildkv-server -- --config ./config.toml
