.PHONY: clean build run

all: build run

build:
	@rustup run nightly -- cargo build

run:
	@rustup run nightly -- cargo run

clean:
	rm -rf target
