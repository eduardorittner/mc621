build: main.c
	clang main.c -o main

test: main.c text build
	cat text | ./main

run: main.c
	clang main.c -o main && ./main
