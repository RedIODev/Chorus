CC = gcc
CFLAGS = -g -Wall

SRC_DIR = src
OBJ_DIR = obj

SRCS = $(shell find * -name '*.c')

OBJS = $(SRCS:$(SRC_DIR)/%.c=$(OBJ_DIR)/%.o)
BIN = main

all: $(OBJS)
	$(CC) $(CFLAGS) $(OBJS) -o main

$(OBJS): $(OBJ_DIR)/%.o : $(SRC_DIR)/%.c 
	mkdir -p $(dir $@)
	$(CC) $(CFLAGS) -c $< -o $@
	@echo "Compiled "$<" successfully!"

clean:
	$(RM) -r main $(OBJ_DIR)/*
