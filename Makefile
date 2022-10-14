all:
	cargo build --release --target=x86_64-unknown-linux-musl
	cp ./target/x86_64-unknown-linux-musl/release/procon_image ./bootstrap
	zip -r lambda.zip ./bootstrap
