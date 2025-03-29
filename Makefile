.PHONY: all build run status plot clean

all: build run

build:
	cargo build --release

run:
	cargo run --release

status:
	cargo run --release -- status

plot:
	cargo run --release -- plot

clean:
	cargo clean
	rm -rf data/*