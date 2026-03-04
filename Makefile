all: compile run

compile:
	cargo build --release

run:
	cargo run

.PHONY: install
install:
	sudo cp ./target/release/silhouette /usr/local/bin/sil
