
all:

build:
	cargo build

test:
	cargo test

run:
	cargo run

env:
	nix-shell shell.nix

