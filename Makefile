CC = gcc
CCFLAGS = -Wall -Wpedantic -Werror -Wextra -Wswitch-enum -Wuse-after-free=3 -Winit-self -Wuninitialized -Wmissing-noreturn -Wshadow=global -Wwrite-strings 
CRFFLAGS = -Wsuggest-attribute=pure -Wsuggest-attribute=const -Wsuggest-attribute=noreturn -Wsuggest-attribute=malloc -Wsuggest-attribute=cold -Wconversion
CDFLAGS = -g3 -Og
CRFLAGS = -g0 -O3
CRFFLAGS += $(CDFLAGS)

SRC_DIR = src
OBJ_DIR = obj

SRCS = $(shell find * -name '*.c')

OBJS = $(SRCS:$(SRC_DIR)/%.c=$(OBJ_DIR)/%.o)

BIN = main
BIN_RELEASE = main-release
TEST_FILE = Testfile.ch

all: CCFLAGS += $(CDFLAGS)
all: $(OBJS) 
	$(CC) $(CCFLAGS) $(OBJS) -o $(BIN)

refactor: CCFLAGS += $(CRFFLAGS)
refactor: all

release: CCFLAGS += $(CRFLAGS)
release: $(OBJS) 
	$(CC) $(CCFLAGS) $(OBJS) -o $(BIN_RELEASE)

run: all
	./$(BIN) $(TEST_FILE)

valgrind: all
	valgrind --tool=massif ./$(BIN) $(TEST_FILE)
	valgrind --leak-check=full --show-leak-kinds=all --track-origins=yes --verbose --log-file=valgrind-out.txt ./$(BIN) $(TEST_FILE)

$(OBJS): $(OBJ_DIR)/%.o : $(SRC_DIR)/%.c 
	mkdir -p $(dir $@)
	$(CC) $(CCFLAGS) -c $< -o $@
	@echo "Compiled "$<" successfully!"

clean:
	$(RM) -r $(BIN) $(BIN_RELEASE) massif.out* valgrind-out.txt $(OBJ_DIR)/*
