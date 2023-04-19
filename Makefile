BIN:=target/release/sudoku
SOURCES = $(wildcard src/*) Cargo.toml

PUZZLES=sudoku.csv
FAILURES=failures.csv

$(BIN): $(SOURCES)
	cargo build -r

$(PUZZLES):
	echo "Download ${PUZZLES} from https://www.kaggle.com/datasets/bryanpark/sudoku"
	exit 1

failures.csv:
	$(MAKE) run-all

.PHONY: run
run: $(BIN) $(PUZZLES)
	cat ${PUZZLES} | parallel --pipe -N1000 ${BIN} | tee ${FAILURES}

.PHONY: run-one
run-one: $(BIN) $(PUZZLES)
	head -2 ${PUZZLES} | ${BIN}

.PHONY: run-failures
run-failures: $(BIN) $(FAILURES)
	cat ${FAILURES} | ${BIN}
