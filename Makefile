
all: format build

clean:
	@cargo clean

format:
	@cargo fmt

build:
	@cargo run -q --package wildkv-server --bin wildkv-server -- --config ./config.toml
