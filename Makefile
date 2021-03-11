all:
	cargo build --release

install:
	install -m755 target/release/catty /usr/local/bin
