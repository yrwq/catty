all:
	cargo build --release

install:
	install -m755 target/release/catty /usr/local/bin

clean:
	rm -rf Cargo.lock target
