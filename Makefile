all: compile run

compile:
	cargo build

run:
	cargo run

.PHONY: install
install:
	sudo cp ./target/debug/silhouette /usr/local/bin/sil
