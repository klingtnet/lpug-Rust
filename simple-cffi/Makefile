.PHONY: clean

PYVER=3

all: target/debug/libsimple_cffi.so

py: all
	@python$(PYVER) square.py

pypy: all
	@pypy$(PYVER) square.py

target/debug/libsimple_cffi.so: src/lib.rs
	cargo build

clean:
	rm -rf target
