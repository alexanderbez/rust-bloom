.PHONY: test bench build

all: test bench build

test:
	@cargo test

bench:
	@cargo bench

build:
	@cargo build
