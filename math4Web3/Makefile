# Define the compiler
CC = gcc

# Define any compiler flags
CFLAGS = -Wall -Wextra

# The target file
TARGET ?= a

# Det the default dependencies
DEPS ?= file1.c file2.c

# The default target
all: $(TARGET)

# Rule to compile the C file
$(TARGET): $(DEPS)
	$(CC) $(CFLAGS) -o $@ $^

# Clean up object files and the executable
clean:
	rm -f $(TARGET) *.o

.PHONY: all clean
