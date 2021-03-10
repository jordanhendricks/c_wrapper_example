.PHONY: all
all:
	cargo build
	cbindgen -l C --output num_wrap.h
	gcc -Wall -ggdb -o num_wrap num_wrap.c target/debug/libc_wrapper_example.so
