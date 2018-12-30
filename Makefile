.PHONY: test bench build

all: test bench build

tests:
	@cargo test

bench:
	@cargo bench

build:
	@cargo build
