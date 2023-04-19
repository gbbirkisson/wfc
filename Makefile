BIN:=target/release/sudoku

SOURCES = $(wildcard src/*) Cargo.toml

$(BIN): $(SOURCES)
	cargo build -r

.PHONY: run-one
run-one: $(BIN)
	head -2 sudoku.csv | parallel --pipe -N1000 $(BIN) | tee failures.csv

.PHONY: run
run: $(BIN)
	cat sudoku.csv | parallel --pipe -N1000 $(BIN) | tee failures.csv

.PHONY: failures
failures: $(BIN)
	cat failures.csv | parallel --pipe -N1000 $(BIN)
