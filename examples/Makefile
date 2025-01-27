ASM_FILES := $(wildcard *.asm)
EXECUTABLES := $(ASM_FILES:.asm=.bin)

BASM_CMD := basm

ifeq ($(OS),Windows_NT)
    RM := del /Q
else
    RM := rm -f
endif

all: clean $(EXECUTABLES)

%.bin: %.asm
	$(BASM_CMD) -o $@ $<

clean:
	$(RM) $(EXECUTABLES) || true

.PHONY: all clean
