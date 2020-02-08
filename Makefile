clean:
	cargo clean
release-arm:
	cargo build --release --target=armv7-unknown-linux-gnueabihf
release-def:
	cargo build --release
