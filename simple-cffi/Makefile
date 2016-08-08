.PHONY: clean

all: target/debug/libsimple_cffi.so

run: all
	@python square.py

target/debug/libsimple_cffi.so: src/lib.rs
	cargo build

clean:
	rm -rf target
