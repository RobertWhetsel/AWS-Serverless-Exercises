.PHONY: build clean

build:
	cargo build --release --target x86_64-unknown-linux-musl
	cp target/x86_64-unknown-linux-musl/release/lambda-handler ./bootstrap

clean:
	cargo clean
	rm -f ./bootstrap
