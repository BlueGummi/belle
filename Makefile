TARGETS = bdump basm belle btils
BIN_DIR = bin

BDUMP_CMD = cd bdump && make
BASM_CMD = cd basm && cargo build --release --quiet 
BELLE_CMD = cd belle && cargo build --release --quiet
BTILS_CMD = cd btils && make

.PHONY: all clean $(TARGETS)

all: $(BIN_DIR) $(TARGETS)

$(BIN_DIR):
	mkdir -p $(BIN_DIR)

bdump:
	$(BDUMP_CMD)
	cp -f bdump/bdump $(BIN_DIR)

basm:
	$(BASM_CMD)
	cp -f basm/target/release/basm $(BIN_DIR)

belle:
	$(BELLE_CMD)
	cp -f belle/target/release/belle $(BIN_DIR)

btils:
	$(BTILS_CMD)
	cp -f btils/bfmt $(BIN_DIR)

clean:
	cd bdump && make clean
	cd basm && cargo clean --quiet
	cd belle && cargo clean --quiet
	cd btils && make clean
	rm -rf site/node_modules

help:
	@echo "Usage: make [TARGETS] [OPTIONS]"
	@echo "Options:"
	@echo "  clean        Clean the build directories"
	@echo "  help         Display this help message"
	@echo "Targets:"
	@echo "  bdump, basm, belle, btils (default: all)"
