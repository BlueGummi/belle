CC = gcc
CFLAGS = -Wall -Wextra -flto -O2 -Wno-cast-function-type
SRCDIR = src
OBJ = bdump.o # print_utils.o # math_functions.o
TARGET = bdump

ifeq ($(OS),Windows_NT)
    RM = del
    RM_FLAGS = /Q
    OBJ_DIR = $(SRCDIR)\*.o
else
    RM = rm -f
    OBJ_DIR = $(SRCDIR)/*.o
endif

RELEASE_FLAGS = -static

.PHONY: all release run clean

all: $(TARGET)

release: CFLAGS += $(RELEASE_FLAGS)
release: $(TARGET)

$(TARGET): $(SRCDIR)/$(OBJ)
	$(CC) $^ -o $@ $(CFLAGS)
	$(RM) $(OBJ_DIR) $(RM_FLAGS) || true

$(SRCDIR)/%.o: $(SRCDIR)/%.c
	$(CC) $(CFLAGS) -c $< -o $@

run: $(TARGET)
	./$(TARGET)

clean:
	$(RM) $(OBJ_DIR) $(RM_FLAGS) || true
	$(RM) $(TARGET) || true
