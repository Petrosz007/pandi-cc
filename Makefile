.POSIX:
.SUFFIXES:
.PHONY: clean
CC     = cc
CFLAGS = -std=c17 -Wall -O
LDLIBS = -lm

all: pandi-cc
pandi-cc: src/main.o
	$(CC) $(LDLIBS) -o pandi-cc src/main.o
src/main.o: src/main.c
	$(CC) -c $(CFLAGS) -o src/main.o src/main.c src/cli.c
src/cli.o: src/cli.c
	$(CC) -c $(CFLAGS) -o src/cli.o src/cli.c
clean:
	rm -f pandi-cc src/main.o test/test_main

test: test/test_cli
	./test/test_cli
test/test_cli: src/cli.o
	$(CC) $(CFLAGS) $(LDLIBS) -o test/test_cli test/test_cli.c src/cli.o
