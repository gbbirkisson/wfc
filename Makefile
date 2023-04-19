BIN:=target/release/sudoku

$(BIN):
	cargo build -r

.PHONY: run
run: $(BIN)
	cat sudoku.csv | parallel --pipe -N1000 $(BIN) | tee failures.csv

.PHONY: failures
failures: $(BIN)
	cat failures.csv | parallel --pipe -N1000 $(BIN)
